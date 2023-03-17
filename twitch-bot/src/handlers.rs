use substring::Substring;
use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

pub async fn irc_message_handler(
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    message: PrivmsgMessage,
) {
    let text = &message.message_text;

    let msg: Option<String> = match text.substring(1, text.len()).trim() {
        _ => None,
    };

    if msg.is_some() {
        client
            .say_in_reply_to(&message, msg.unwrap())
            .await
            .expect("Unable to send a message");
    }
}
