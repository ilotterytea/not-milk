#![feature(exclusive_range_pattern)]
use diesel::{QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use infrastructure::establish_connection;
use std::env;
use twitch_api::{
    helix::users::GetUsersRequest,
    twitch_oauth2::{AccessToken, UserToken},
    types::UserIdRef,
    TwitchClient,
};
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

mod handlers;
mod smol_commands;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let config = if env::var("TWITCH_OAUTH2_TOKEN").is_ok() && env::var("TWITCH_BOT_NAME").is_ok() {
        ClientConfig::new_simple(StaticLoginCredentials::new(
            env::var("TWITCH_BOT_NAME").unwrap(),
            Some(env::var("TWITCH_OAUTH2_TOKEN").unwrap()),
        ))
    } else {
        ClientConfig::default()
    };

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    if env::var("TWITCH_INITIAL_CHANNEL_NAMES").is_ok() {
        for name in env::var("TWITCH_INITIAL_CHANNEL_NAMES").unwrap().split(',') {
            client.join(name.to_owned()).unwrap();
        }
    }

    let conn = &mut establish_connection();
    let channels = infrastructure::schema::channels::dsl::channels
        .select(infrastructure::schema::channels::dsl::alias_id)
        .load::<i32>(conn)
        .expect("Couldn't get the channels!");

    let api_client: TwitchClient<reqwest::Client> = TwitchClient::default();

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

    for id in channels {
        let _id = id.to_string();
        let ids: &[&UserIdRef] = &[_id.as_str().into()];

        let users = &api_client
            .helix
            .req_get(GetUsersRequest::ids(ids), &token)
            .await
            .expect("Couldn't send a request!");

        let user = users.data.first();

        if user.is_some() {
            let u = user.unwrap();

            client.join(u.login.to_string()).unwrap();
        }
    }

    let main_handler = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                handlers::irc_message_handler(&client, msg).await;
            }
        }
    });

    main_handler.await.unwrap();
}
