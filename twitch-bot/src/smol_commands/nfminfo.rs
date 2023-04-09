use std::env;

use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, NonFungibleMilk},
    schema::{actions::dsl as act, consumers::dsl as cs, non_fungible_milks::dsl as nfm},
};
use rand::Rng;

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "nfminfo",
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
        return Some(format!("{}: no nfm id provided!", consumer.alias_name));
    }

    if msg_args.message.as_ref().unwrap().parse::<u32>().is_err() {
        return Some(format!(
            "{}: the provided id ({}) is not a nfm id!",
            consumer.alias_name,
            msg_args.message.as_ref().unwrap()
        ));
    }

    let id = msg_args.message.as_ref().unwrap().parse::<u32>().unwrap();

    let _nfm = nfm::non_fungible_milks
        .filter(nfm::id.eq(id as i32))
        .filter(nfm::sold.eq(0))
        .first::<NonFungibleMilk>(conn);

    if _nfm.is_err() {
        return Some(format!(
            "{}: this nfm doesn't exist or has been sold!",
            consumer.alias_name
        ));
    }

    let milk = _nfm.unwrap();
    let rarity_percent = (milk.rarity as f32 / milk.hash_sum.len() as f32) * 100.0;
    let timestamp = chrono::Utc::now().timestamp() as i32 - milk.created_at;
    let nfm_delta = env::var("NFM_DELTA")
        .unwrap_or(((rand::thread_rng().gen_range(-1.0..100.0)) / 10000.0).to_string())
        .parse::<f32>()
        .unwrap();

    let nfm_cost = env::var("NFM_COST")
        .unwrap_or("800".to_string())
        .parse::<i32>()
        .unwrap();

    let sell_cost = nfm_cost as f32
        + ((nfm_delta * (milk.rarity as f32 / if milk.rarity < 10 { 1.0 } else { 10.0 }))
            * timestamp as f32);

    let target_consumer = cs::consumers
        .find(milk.consumer_id)
        .first::<Consumer>(conn)
        .expect("Cannot get a consumer");

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

    Some(format!(
        "{}: NFM #{} (owner: @{}) -> rarity: {} ({}), purchased {} ago, best price to sell: {} 🥛 ",
        consumer.alias_name,
        milk.id,
        target_consumer.alias_name,
        if rarity_percent >= 80.0 {
            "LEGENDARY"
        } else if (70.0..80.0).contains(&rarity_percent) {
            "Epic"
        } else if (60.0..70.0).contains(&rarity_percent) {
            "Rare"
        } else {
            "Common"
        },
        milk.rarity,
        humanize_timestamp_like_timer(timestamp),
        sell_cost
    ))
}
