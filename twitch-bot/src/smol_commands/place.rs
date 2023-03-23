use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, Savegame},
    schema::{actions::dsl as act, savegames::dsl as sg},
};

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub async fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("place"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if _latest_action_timestamp.is_ok() {
        let latest_action_timestamp = _latest_action_timestamp.unwrap();
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - latest_action_timestamp;
        let delay = std::env::var("PLACE_DELAY_SEC")
            .unwrap_or("10".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: i am sorry, Master... but you have to wait {} for me to show you your place 🥛 ✋ ",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
            ));
        }
    }

    let savegames = sg::savegames
        .order(sg::points.desc())
        .load::<Savegame>(conn)
        .expect("Couldn't get the savegames!");

    let pos = savegames
        .iter()
        .position(|p| p.consumer_id.eq(&consumer.id))
        .unwrap();

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "place",
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
        "{}: you're in #{} place out of {}! {} {}",
        consumer.alias_name,
        pos + 1,
        savegames.len(),
        match pos {
            0 => "🥇 ",
            1 => "🥈 ",
            2 => "🥉 ",
            _ => "🏆 ",
        },
        if pos.ne(&0) {
            let next_savegame = savegames.get(pos - 1).unwrap();
            let my_savegame = savegames.get(pos).unwrap();

            let difference = next_savegame.points - my_savegame.points;

            format!(
                "| you need {} more 🥛 to level up your place on the leaderboard! forsenLevel",
                difference + 1
            )
        } else {
            "".to_string()
        }
    ))
}
