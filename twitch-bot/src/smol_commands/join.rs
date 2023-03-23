use std::env;

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Channel, Consumer, NewAction, NewChannel},
    schema::{actions::dsl as act, channels::dsl as ch},
};
use twitch_api::{
    helix::users::GetUsersRequest,
    twitch_oauth2::{AccessToken, UserToken},
    types::UserIdRef,
    TwitchClient,
};
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

use crate::utils::ParsedMessage;

pub async fn run(
    consumer: Consumer,
    msg_args: &ParsedMessage,
    twitch_client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) -> Option<String> {
    let conn = &mut establish_connection();

    let channel = ch::channels
        .filter(ch::alias_id.eq(consumer.alias_id))
        .first::<Channel>(conn);

    let client: TwitchClient<reqwest::Client> = TwitchClient::default();

    let token = match UserToken::from_existing(
        &reqwest::Client::new(),
        AccessToken::new(
            env::var("TWITCH_ACCESS_TOKEN")
                .expect("TWITCH_ACCESS_TOKEN must be set for Twitch API requests!"),
        ),
        None,
        None,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => panic!("Got error: {}", e),
    };

    let _id = consumer.alias_id.to_string();
    let ids: &[&UserIdRef] = &[_id.as_str().into()];

    let users = &client
        .helix
        .req_get(GetUsersRequest::ids(ids), &token)
        .await
        .expect("Couldn't send a request!");

    let user = users.data.first()?;

    if channel.is_ok() {
        let c = channel.unwrap();

        if c.is_parted.eq(&1) {
            update(ch::channels.find(c.id))
                .set(ch::is_parted.eq(&0))
                .execute(conn)
                .expect("Couldn't update the 'is_parted' status!");

            twitch_client.join(user.login.to_string()).unwrap();
            return Some(format!(
                "ok, i restored the 'not milk' supply to the #{} chat room!",
                user.login
            ));
        }

        return Some(format!(
            "i'm already supplying 'not milk' to the #{} chat room!",
            user.login
        ));
    }

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "join",
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

    insert_into(ch::channels)
        .values(vec![NewChannel {
            alias_id: user.id.to_string().parse::<i32>().unwrap(),
            joined_at: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
        }])
        .execute(conn)
        .expect("Couldn't add a new channel!");

    twitch_client.join(user.login.to_string()).unwrap();

    Some(format!(
        "ok, i'll now supply 'not milk' to the #{} chat room!",
        user.login
    ))
}
