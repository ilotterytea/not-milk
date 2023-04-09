use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction},
    schema::{actions::dsl as act, non_fungible_milks::dsl as nfm},
};

use crate::utils::ParsedMessage;

pub fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "mynfms",
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

    let _nfms = nfm::non_fungible_milks
        .filter(nfm::consumer_id.eq(consumer.id))
        .filter(nfm::sold.eq(0))
        .order(nfm::created_at.asc())
        .select(nfm::id)
        .load::<i32>(conn);

    if _nfms.is_err() {
        return Some(format!("{}: something went wrong! maybe you don't have any nfms or this is just a random error...", consumer.alias_name));
    }

    let nfms = _nfms.unwrap();

    Some(format!(
        "{}: your nfms: {}",
        consumer.alias_name,
        nfms.iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    ))
}
