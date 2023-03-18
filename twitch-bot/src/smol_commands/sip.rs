use std::{cmp::Ordering, env};

use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewPointsHistory, NewSavegame, PointsHistory, Savegame, Suspension},
    schema::{
        consumers::dsl as cs, lines::dsl as ln, points_history::dsl as ph, savegames::dsl as sg,
        suspensions::dsl as sus,
    },
};

use rand::Rng;

use crate::utils::{humanize_timestamp_like_timer, sync_consumer};

pub async fn run(user_id: &str) -> Option<String> {
    // Getting a 'Not milk' information about user from provided user_id:
    let conn = &mut establish_connection();
    let consumer = sync_consumer(user_id).await?;

    let _suspension = sus::suspensions.find(consumer.id).first::<Suspension>(conn);

    if _suspension.is_ok() {
        let suspension = _suspension.unwrap();

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

    let histories = ph::points_history
        .filter(ph::consumer_id.eq(consumer.id))
        .order(ph::timestamp.desc())
        .load::<PointsHistory>(conn)
        .expect("Couldn't load activities!");

    if !histories.is_empty() {
        let latest = histories.first().unwrap();

        let difference = i32::try_from(chrono::Utc::now().timestamp()).unwrap() - latest.timestamp;
        let delay = env::var("INTERVAL_SEC")
            .unwrap_or("3600".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: please wait! i need {} to make another milk for you 🫙  😣",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
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
    let mut points = rand::thread_rng().gen_range(min..max);
    let mut percent = (points as f64 / max as f64) * 100.0;

    let message = match fun {
        // very very negative event:
        0 => {
            points = rand::thread_rng().gen_range(-99..-50);
            "you were cursed with an intolerance to my milk 🥛😭".to_string()
        }
        // Steal from random consumer:
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

            percent = (points as f64 / max as f64) * 100.0;
            format!(
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
            )
        }
        // Regular event:
        _ => {
            let category_id = if percent > 75.0 {
                3
            } else if (50.0..75.0).contains(&percent) {
                2
            } else if (25.0..50.0).contains(&percent) {
                1
            } else {
                0
            };

            let lines = ln::lines
                .filter(ln::category_id.eq(category_id))
                .select(ln::line)
                .load::<String>(conn)
                .expect("Couldn't load the lines!");

            if lines.is_empty() {
                "missingno".to_string()
            } else {
                let index = rand::thread_rng().gen_range(0..lines.len());

                lines.get(index).unwrap().to_owned()
            }
        }
    };

    // Update the sender's points:
    savegame.points += points;

    update(sg::savegames.find(savegame.consumer_id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame");

    // Create a new activity:
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
