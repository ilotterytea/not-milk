use substring::Substring;
use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

use crate::smol_commands;

pub async fn irc_message_handler(
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    message: PrivmsgMessage,
) {
    let text = &message.message_text;

    if text.starts_with("🥛 ") {
        let msg: Option<String> = match text.substring(1, text.len()).trim() {
            "sip" => smol_commands::sip::run(message.sender.id.clone()).await,
            _ => None,
        };

        if msg.is_some() {
            client
                .say_in_reply_to(&message, msg.unwrap())
                .await
                .expect("Unable to send a message");
        }
    }
}
