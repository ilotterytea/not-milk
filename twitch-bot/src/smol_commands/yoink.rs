use std::env;

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};
use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Action, Consumer, NewAction, NewPointsHistory, Savegame},
    schema::{
        actions::dsl as act, consumers::dsl as cs, points_history::dsl as ph, savegames::dsl as sg,
    },
};
use rand::Rng;

pub async fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    let _action = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("yoink"))
        .order(act::created_at.desc())
        .first::<Action>(conn);

    if _action.is_ok() {
        let action = _action.unwrap();
        let difference = i32::try_from(chrono::Utc::now().timestamp()).unwrap() - action.created_at;
        let delay = env::var("YOINK_DELAY_SEC")
            .unwrap_or("900".to_string())
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

    let target_consumer = if msg_args.message.is_some() {
        let c = cs::consumers
            .filter(cs::alias_name.eq(msg_args.message.as_ref().unwrap().to_lowercase()))
            .first::<Consumer>(conn);

        if c.is_err() {
            return Some(format!(
                "{}: User with the nickname '{}' not found!",
                consumer.alias_name,
                msg_args.message.as_ref().unwrap()
            ));
        }

        c.unwrap()
    } else {
        let all_consumers = cs::consumers
            .filter(cs::alias_id.ne(consumer.alias_id))
            .load::<Consumer>(conn)
            .expect("Couldn't load the consumers!");

        let index = rand::thread_rng().gen_range(0..all_consumers.len());

        all_consumers.get(index).cloned().unwrap()
    };

    let mut target_savegame = sg::savegames
        .find(target_consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't find the savegame!");

    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't find the savegame!");

    let points = rand::thread_rng().gen_range(1..50);
    let fun = rand::thread_rng().gen_range(0..100);
    let message = match fun {
        (0..15) => {
            savegame.points -= points;
            update(sg::savegames.find(consumer.id))
                .set(sg::points.eq(savegame.points))
                .execute(conn)
                .expect("Couldn't update the savegame!");

            insert_into(ph::points_history)
                .values(vec![NewPointsHistory {
                    consumer_id: consumer.id,
                    caused_by_consumer_id: None,
                    difference: points,
                    points_before_difference: savegame.points - points,
                    timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                }])
                .execute(conn)
                .expect("Couldn't insert new history of the points");
            format!("{}: while u were trying to sneak up on {}, you slipped on spilled 🥛 and dropped {} of your 🥛 collection ... epic fail lol", consumer.alias_name, target_consumer.alias_name, points)
        }
        (15..30) => {
            savegame.points -= points;
            target_savegame.points += points;

            update(sg::savegames.find(consumer.id))
                .set(sg::points.eq(savegame.points))
                .execute(conn)
                .expect("Couldn't update the savegame!");

            update(sg::savegames.find(target_consumer.id))
                .set(sg::points.eq(target_savegame.points))
                .execute(conn)
                .expect("Couldn't update the savegame!");

            insert_into(ph::points_history)
                .values(vec![
                    NewPointsHistory {
                        consumer_id: consumer.id,
                        caused_by_consumer_id: Some(target_consumer.id),
                        difference: points,
                        points_before_difference: savegame.points + points,
                        timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                    },
                    NewPointsHistory {
                        consumer_id: target_consumer.id,
                        caused_by_consumer_id: None,
                        difference: points,
                        points_before_difference: target_savegame.points - points,
                        timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                    },
                ])
                .execute(conn)
                .expect("Couldn't insert new history of the points");
            format!("{}: you tried to apporach {} quietly, but they heard you, took you by force, and pumped {} 🥛 out of you!!! 🥴 ", consumer.alias_name, target_consumer.alias_name, points)
        }
        (30..75) => format!("{}: no 'not milk' found... 😮‍💨", consumer.alias_name),
        _ => {
            savegame.points += points;
            target_savegame.points -= points;

            update(sg::savegames.find(consumer.id))
                .set(sg::points.eq(savegame.points))
                .execute(conn)
                .expect("Couldn't update the savegame!");

            update(sg::savegames.find(target_consumer.id))
                .set(sg::points.eq(target_savegame.points))
                .execute(conn)
                .expect("Couldn't update the savegame!");

            insert_into(ph::points_history)
                .values(vec![
                    NewPointsHistory {
                        consumer_id: consumer.id,
                        caused_by_consumer_id: None,
                        difference: points,
                        points_before_difference: savegame.points - points,
                        timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                    },
                    NewPointsHistory {
                        consumer_id: target_consumer.id,
                        caused_by_consumer_id: Some(consumer.id),
                        difference: points,
                        points_before_difference: target_savegame.points + points,
                        timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                    },
                ])
                .execute(conn)
                .expect("Couldn't insert new history of the points");

            format!(
                "{}: you jumped on {} and were able to suck {} 🥛 out of them... 😋 ",
                consumer.alias_name, target_consumer.alias_name, points
            )
        }
    };

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "yoink",
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

    Some(message)
}
