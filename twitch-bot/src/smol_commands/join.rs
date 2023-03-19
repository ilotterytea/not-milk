use std::env;

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Channel, NewChannel},
    schema::channels::dsl as ch,
};
use twitch_api::{
    helix::users::GetUsersRequest,
    twitch_oauth2::{AccessToken, UserToken},
    types::UserIdRef,
    TwitchClient,
};
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

pub async fn run(
    user_id: &str,
    twitch_client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) -> Option<String> {
    let conn = &mut establish_connection();

    let channel = ch::channels
        .filter(ch::alias_id.eq(user_id.parse::<i32>().unwrap()))
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

    let ids: &[&UserIdRef] = &[user_id.into()];

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
