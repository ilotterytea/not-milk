use std::env;

use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, Savegame},
    schema::savegames::dsl as sg,
};
use rand::Rng;
use substring::Substring;

use crate::utils::ParsedMessage;

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    if msg_args.message.is_none() {
        return Some(format!(
            "{}: no percent or number is given!",
            consumer.alias_name
        ));
    }
    let msg = msg_args.message.as_ref().unwrap();

    let percent = msg.ends_with('%');
    let conn = &mut establish_connection();
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
