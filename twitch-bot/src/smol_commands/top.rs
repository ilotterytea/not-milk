use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, Savegame},
    schema::{actions::dsl as act, consumers::dsl as cs, savegames::dsl as sg},
};

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    let latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("top"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if let Ok(latest_action_timestamp) = latest_action_timestamp {
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - latest_action_timestamp;
        let delay = std::env::var("TOP_DELAY_SEC")
            .unwrap_or("60".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: i am sorry, Master... but you have to wait {} for me to show you the leaderboard 🥛 ✋ ",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
            ));
        }
    }

    let savegames = sg::savegames
        .order(sg::points.desc())
        .limit(5)
        .load::<Savegame>(conn)
        .expect("Couldn't get the savegames!");

    let mut strings: Vec<String> = vec![];
    let mut index = 0;

    for savegame in &savegames {
        let consumer = cs::consumers
            .find(savegame.consumer_id)
            .first::<Consumer>(conn)
            .expect("Couldn't get the consumer!");

        index += 1;

        strings.push(format!(
            "{} {}. {} ({})",
            match index {
                1 => "🥇 ",
                2 => "🥈 ",
                3 => "🥉 ",
                _ => "",
            },
            index,
            consumer.alias_name,
            savegame.points
        ));
    }

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "top",
            body: if msg_args.message.is_some() {
                Some(msg_args.message.as_ref().unwrap().as_str())
            } else {
                None
            },
            raw: msg_args.raw_message.as_str(),
            created_at: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
        }])
        .execute(conn)
        .expect("Couldn't insert a new action!");

    Some(format!(
        "🥛 🏆 top {} milk sippers: {}",
        index,
        strings.join(", ")
    ))
}
