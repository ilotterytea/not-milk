use dotenvy::dotenv;
use std::env;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

mod handlers;
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

    let main_handler = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                handlers::irc_message_handler(&client, msg).await;
            }
        }
    });

    main_handler.await.unwrap();
}
