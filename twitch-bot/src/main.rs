use std::env;
use dotenvy::dotenv;
use twitch_irc::{ClientConfig, login::StaticLoginCredentials, TwitchIRCClient, SecureTCPTransport, message::ServerMessage};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let config = if env::var("TWITCH_OAUTH2_TOKEN").is_ok() && env::var("TWITCH_BOT_NAME").is_ok() {
        ClientConfig::new_simple(StaticLoginCredentials::new(env::var("TWITCH_BOT_NAME").unwrap(), Some(env::var("TWITCH_OAUTH2_TOKEN").unwrap())))
    } else {
        ClientConfig::default()
    };

    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    if env::var("TWITCH_INITIAL_CHANNEL_NAMES").is_ok() {
        for name in env::var("TWITCH_INITIAL_CHANNEL_NAMES").unwrap().split(',') {
            client.join(name.to_owned()).unwrap();
        }
    }

    let main_handler = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                println!("{:?}", msg);
            }
        }
    });

    main_handler.await.unwrap();
}
