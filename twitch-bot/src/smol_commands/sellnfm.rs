use std::env;

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, NonFungibleMilk, Savegame},
    schema::{actions::dsl as act, non_fungible_milks::dsl as nfm, savegames::dsl as sg},
};
use rand::Rng;

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("sellnfm"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if let Ok(_latest_action_timestamp) = _latest_action_timestamp {
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - _latest_action_timestamp;
        let delay = std::env::var("SELLNFM_DELAY_SEC")
            .unwrap_or("30".to_string())
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
            name: "sellnfm",
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

    let id: i32 = if msg_args.message.is_some() {
        let m = msg_args.message.as_ref().unwrap();

        let id = m.parse::<i32>();

        if id.is_err() {
            return Some(format!(
                "{}: i can't parse the id from '{}' 😠",
                consumer.alias_name, m
            ));
        }

        id.unwrap()
    } else {
        return Some(format!("{}: id not specified! 😠", consumer.alias_name));
    };

    let _nfm = nfm::non_fungible_milks
        .find(id)
        .filter(nfm::sold.ne(1))
        .first::<NonFungibleMilk>(conn);

    if _nfm.is_err() {
        return Some(format!(
            "{}: NFM with ID {} does not exist! 😠",
            consumer.alias_name, id
        ));
    }

    let nfmilk = _nfm.unwrap();

    if nfmilk.consumer_id.ne(&consumer.id) {
        return Some(format!(
            "{}: you don't own this nfm 😠",
            consumer.alias_name
        ));
    }

    let difference = i32::try_from(chrono::Utc::now().timestamp()).unwrap() - nfmilk.created_at;

    if difference < 86400 {
        return Some(format!(
            "{}: you can sell this nfm 24 hours after purchase ({} left) 🥛 💰 ✋ ",
            consumer.alias_name,
            humanize_timestamp_like_timer(86400 - difference)
        ));
    }

    let nfm_delta = env::var("NFM_DELTA")
        .unwrap_or(((rand::thread_rng().gen_range(-1.0..100.0)) / 10000.0).to_string())
        .parse::<f32>()
        .unwrap();

    let nfm_cost = env::var("NFM_COST")
        .unwrap_or("800".to_string())
        .parse::<i32>()
        .unwrap();

    let sell_cost = nfm_cost as f32
        + ((nfm_delta * (nfmilk.rarity as f32 / if nfmilk.rarity < 10 { 1.0 } else { 10.0 }))
            * difference as f32);

    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't find the savegame!");

    savegame.points += sell_cost.round() as i32;

    update(sg::savegames.find(consumer.id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame!");

    if env::var("NFM_DELTA").is_err()
        || env::var("NFM_DELTA_LAST_CHANGE").is_err()
        || env::var("NFM_DELTA_LAST_CHANGE")
            .unwrap()
            .ne(&chrono::Utc::now().date_naive().to_string())
    {
        env::set_var("NFM_DELTA", nfm_delta.to_string());
        env::set_var(
            "NFM_DELTA_LAST_CHANGE",
            chrono::Utc::now().date_naive().to_string(),
        );
    }

    update(nfm::non_fungible_milks.find(nfmilk.id))
        .set(nfm::sold.eq(1))
        .execute(conn)
        .expect("Couldn't update the NFM!");

    Some(format!(
        "{}: successfully sold NFM ID {}! you have earned +{} 🥛 📈",
        consumer.alias_name,
        id,
        sell_cost.round() as i32
    ))
}
