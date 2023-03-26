use std::env;

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, Savegame},
    schema::{actions::dsl as act, savegames::dsl as sg},
};
use rand::Rng;
use substring::Substring;

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();
    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("roulette"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if let Ok(_latest_action_timestamp) = _latest_action_timestamp {
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - _latest_action_timestamp;
        let delay = std::env::var("ROULETTE_DELAY_SEC")
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

    if msg_args.message.is_none() {
        return Some(format!(
            "{}: no percent or number is given!",
            consumer.alias_name
        ));
    }
    let msg = msg_args.message.as_ref().unwrap();

    let percent = msg.ends_with('%');
    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't load the savegame!");

    if savegame.points <= 0 {
        return Some(format!(
            "{}: your balance is less than or equal to 0!",
            consumer.alias_name
        ));
    }

    let points = if percent {
        let m = msg.substring(0, msg.len() - 1).parse::<u32>();
        if m.is_err() {
            return Some(format!(
                "{}: i can't parse the percent from '{}'",
                consumer.alias_name, msg
            ));
        }

        let p = m.unwrap();

        if p > 100 {
            return Some(format!(
                "{}: the percent is more than 100%",
                consumer.alias_name
            ));
        }

        savegame.points as f32 * (p as f32 / 100.0)
    } else if msg.eq(&"all") {
        savegame.points as f32
    } else {
        let m = msg.parse::<u32>();
        if m.is_err() {
            return Some(format!(
                "{}: i can't parse the number from '{}'",
                consumer.alias_name, msg
            ));
        }

        let p = m.unwrap();

        if p as i32 > savegame.points {
            return Some(format!(
                "{}: the number is greater than your 🥛 balance!",
                consumer.alias_name
            ));
        }

        p as f32
    };

    let fun = rand::thread_rng().gen_range(0..100);
    let fun_success = env::var("ROULETTE_SUCCESS")
        .unwrap_or("50".to_string())
        .parse::<u32>()
        .unwrap();

    if fun <= fun_success {
        savegame.points += points.round() as i32;
    } else {
        savegame.points -= points.round() as i32;
    }

    update(sg::savegames.find(consumer.id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame!");

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "roulette",
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
        "{}: you {} {} 🥛 {}",
        consumer.alias_name,
        if fun <= fun_success { "WON" } else { "LOST" },
        points,
        if fun <= fun_success {
            "... 🥛 roulette all NOW!!! ZeroPurpose"
        } else {
            "lol"
        }
    ))
}
