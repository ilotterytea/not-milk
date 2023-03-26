use std::{cmp::Ordering, env, fs::File};

use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, NewPointsHistory, NewSavegame, Savegame, Suspension},
    schema::{
        actions::dsl as act, consumers::dsl as cs, points_history::dsl as ph, savegames::dsl as sg,
        suspensions::dsl as sus,
    },
};

use rand::Rng;

use crate::{
    structs::Lines,
    utils::{humanize_timestamp_like_timer, ParsedMessage},
};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    // Getting a 'Not milk' information about user from provided user_id:
    let conn = &mut establish_connection();

    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("sip"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if let Ok(_latest_action_timestamp) = _latest_action_timestamp {
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - _latest_action_timestamp;
        let delay = std::env::var("SIP_DELAY_SEC")
            .unwrap_or("1500".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: i am sorry, Master... but you have to wait {} for me to pour you a milk 🥛 ✋ ",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
            ));
        }
    }

    let suspension = sus::suspensions.find(consumer.id).first::<Suspension>(conn);

    if let Ok(suspension) = suspension {
        let passed_time =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - suspension.timestamp;

        if passed_time > suspension.duration && suspension.duration < 0 {
            delete(sus::suspensions.find(suspension.consumer_id))
                .execute(conn)
                .expect("Couldn't delete the suspension!");
        } else {
            return Some(format!(
                "{}: sorry, master.. b-but you {} {} 🥛🚫 😭 ",
                consumer.alias_name,
                if suspension.duration < 0 {
                    "have been permanently banned".to_string()
                } else {
                    format!(
                        "were timed out for {}",
                        humanize_timestamp_like_timer(suspension.duration - passed_time)
                    )
                },
                if suspension.reason.is_none() {
                    "for being a 'not milk' denier".to_string()
                } else {
                    format!("for {}", suspension.reason.unwrap())
                }
            ));
        }
    }

    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .unwrap_or_else(|_| {
            insert_into(sg::savegames)
                .values(vec![NewSavegame {
                    consumer_id: consumer.id,
                }])
                .execute(conn)
                .expect("Couldn't insert the values");
            sg::savegames
                .find(consumer.id)
                .first::<Savegame>(conn)
                .unwrap()
        });

    let savegames = sg::savegames
        .order(sg::points.desc())
        .load::<Savegame>(conn)
        .expect("Couldn't load the savegames!");

    let min = env::var("MIN_PER_SIP")
        .unwrap_or("-10".to_string())
        .parse::<i32>()
        .unwrap();
    let max = env::var("MAX_PER_SIP")
        .unwrap_or("50".to_string())
        .parse::<i32>()
        .unwrap();

    let fun = rand::thread_rng().gen_range(0..100);
    let points: i32;
    let message: String;

    match fun {
        // super negative event:
        0 => {
            points = -99;
            message = "you were cursed with an intolerance to my milk 🥛 😭 ".to_string();
        }
        // steal from random consumer
        (1..10) => {
            let mut savegames = sg::savegames
                .filter(sg::consumer_id.ne(savegame.consumer_id))
                .load::<Savegame>(conn)
                .expect("Couldn't retrieve the rows!");

            let index = rand::thread_rng().gen_range(0..savegames.len());
            let _savegame = savegames.get_mut(index).unwrap();

            points = rand::thread_rng().gen_range(1..max);
            _savegame.points -= points;

            update(sg::savegames.find(_savegame.consumer_id))
                .set(sg::points.eq(_savegame.points))
                .execute(conn)
                .expect("Couldn't update the row!");

            let _consumer = cs::consumers
                .find(_savegame.consumer_id)
                .first::<Consumer>(conn)
                .expect("Couldn't get the consumer!");

            insert_into(ph::points_history)
                .values(vec![NewPointsHistory {
                    consumer_id: _consumer.id,
                    caused_by_consumer_id: Some(consumer.id),
                    difference: points,
                    points_before_difference: _savegame.points + points,
                    timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                }])
                .execute(conn)
                .expect("Couldn't create a new activity record!");

            let percent = (points as f64 / max as f64) * 100.0;
            message = format!(
                "you didn't get your portion of milk, but you {} {} {}",
                if percent >= 50.0 {
                    "rudely took"
                } else {
                    "gently asked"
                },
                _consumer.alias_name,
                if percent >= 80.0 {
                    "and made him about to milk 🥛😫"
                } else if (50.0..80.0).contains(&percent) {
                    "and pumped all the milk out of him 🥛🥴"
                } else {
                    "to pour you 'not milk', and he agreed 🥛😊"
                }
            );
        }
        // found something +.
        (11..50) => {
            points = rand::thread_rng().gen_range(1..max);
            let percent = (points as f32 / max as f32) * 100.0;

            let file = File::open("lines.json").unwrap();
            let lines: Lines = serde_json::from_reader(file).unwrap();

            let category = if percent > 75.0 {
                &lines.legendary_lines
            } else if (45.0..75.0).contains(&percent) {
                &lines.epic_lines
            } else {
                &lines.common_lines
            };

            if category.is_empty() {
                message = "missingno".to_string();
            } else {
                let index = rand::thread_rng().gen_range(0..category.len());
                message = category.get(index).unwrap().to_string();
            }
        }
        // found something -.
        (51..80) => {
            points = rand::thread_rng().gen_range(min..-1);

            let file = File::open("lines.json").unwrap();
            let lines: Lines = serde_json::from_reader(file).unwrap();

            let category = &lines.poor_lines;

            if category.is_empty() {
                message = "missingno".to_string();
            } else {
                let index = rand::thread_rng().gen_range(0..category.len());
                message = category.get(index).unwrap().to_string();
            }
        }
        // nothing found.
        _ => {
            points = 0;
            message = "nothing found pls dont yell at me 🥺 🥺 ".to_string();
        }
    }

    // Update the sender's points:
    savegame.points += points;

    update(sg::savegames.find(savegame.consumer_id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame");

    // Create a new activity:
    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "sip",
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

    insert_into(ph::points_history)
        .values(vec![NewPointsHistory {
            consumer_id: consumer.id,
            caused_by_consumer_id: None,
            difference: points,
            points_before_difference: savegame.points - points,
            timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
        }])
        .execute(conn)
        .expect("Couldn't create a new activity record!");

    let savegames_now = sg::savegames
        .order(sg::points.desc())
        .load::<Savegame>(conn)
        .expect("Couldn't load the savegames!");

    let position_then = savegames
        .iter()
        .position(|p| p.consumer_id.eq(&consumer.id))
        .unwrap();

    let position_now = savegames_now
        .iter()
        .position(|p| p.consumer_id.eq(&consumer.id))
        .unwrap();

    if points == 0 {
        return Some(format!("{}: {} ...", consumer.alias_name, message));
    }

    Some(format!(
        "{}: {} ... anyways you got {} | total: {} 🥛{}",
        consumer.alias_name,
        message,
        if points > 0 {
            format!("+{}", points)
        } else {
            points.to_string()
        },
        savegame.points,
        match position_now.cmp(&position_then) {
            Ordering::Less =>
                format!(
                "| forsenLevel ⬆  you have leveled up from #{} place to #{} place in the top of {} sippers!!!",
                position_then + 1,
                position_now + 1,
                savegames_now.len()
            ),
            Ordering::Greater =>
                format!(
                "| forsenLevel ⬇  you lowered from #{} place to #{} place in the top of {} sippers...epic fai.l",
                position_then + 1,
                position_now + 1,
                savegames_now.len()
            ),
            Ordering::Equal => "".to_string(),
        }
    ))
}
