use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, Savegame},
    schema::{actions::dsl as act, consumers::dsl as cs, savegames::dsl as sg},
};

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();
    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("give"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if let Ok(_latest_action_timestamp) = _latest_action_timestamp {
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - _latest_action_timestamp;
        let delay = std::env::var("GIVE_DELAY_SEC")
            .unwrap_or("60".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: i am sorry, Master... but you have to wait {} for me 🥛 ✋ ",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
            ));
        }
    }

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "give",
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

    if msg_args.message.is_none() {
        return Some(format!(
            "{}: invalid syntax! example: 🥛 give AMOUNT to USERNAME",
            consumer.alias_name
        ));
    }

    let args = msg_args
        .message
        .as_ref()
        .unwrap()
        .split(" to ")
        .collect::<Vec<&str>>();

    if args.len() != 2 {
        return Some(format!(
            "{}: invalid syntax! example: 🥛 give AMOUNT to USERNAME",
            consumer.alias_name
        ));
    }

    let points = args.first().unwrap();

    if points.ne(&"*") && points.ne(&"all") && points.parse::<u32>().is_err() {
        return Some(format!(
            "{}: invalid syntax (the amount must be numeric and not be a negative number)! example: 🥛 give 10 to USERNAME",
            consumer.alias_name
        ));
    }

    let username = args.get(1).unwrap();

    let _target_consumer = cs::consumers
        .filter(cs::alias_name.eq(username.to_lowercase()))
        .first::<Consumer>(conn);

    if _target_consumer.is_err() {
        return Some(format!(
            "{}: the user '{}' does not exist!",
            consumer.alias_name, username
        ));
    }

    let target_consumer = _target_consumer.unwrap();

    if target_consumer.id.eq(&consumer.id) {
        return Some(format!(
            "{}: ok what's the point of giving 🥛 to yourself?",
            consumer.alias_name
        ));
    }

    let mut target_savegame = sg::savegames
        .find(target_consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't find the savegame!");

    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't find the savegame!");

    if savegame.points <= 0 {
        return Some(format!(
            "{}: your 🥛  balance is less than or equal to 0!",
            consumer.alias_name
        ));
    }

    let mut _points = savegame.points as u32;

    target_savegame.points += if points.parse::<u32>().is_err() {
        savegame.points
    } else {
        _points = points.parse::<u32>().unwrap();
        _points as i32
    };

    savegame.points -= if points.parse::<u32>().is_err() {
        savegame.points
    } else {
        _points = points.parse::<u32>().unwrap();
        _points as i32
    };

    update(sg::savegames.find(consumer.id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame!");

    update(sg::savegames.find(target_consumer.id))
        .set(sg::points.eq(target_savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame!");

    Some(format!(
        "{}: successfully transfered {} 🥛 to {}",
        consumer.alias_name, _points, target_consumer.alias_name
    ))
}
