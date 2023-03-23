use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

use crate::{smol_commands, utils::ParsedMessage};

pub async fn irc_message_handler(
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    message: PrivmsgMessage,
) {
    let text = &message.message_text;

    let msg_args = ParsedMessage::parse(text.as_str(), '🥛');

    if msg_args.is_some() {
        let _msg_args = msg_args.as_ref().unwrap();

        let msg: Option<String> = match _msg_args.command_id.as_str() {
            "sip" => smol_commands::sip::run(message.sender.id.as_str()).await,
            "place" => smol_commands::place::run(message.sender.id.as_str()).await,
            "top" => smol_commands::top::run(),
            "join" => smol_commands::join::run(message.sender.id.as_str(), client).await,
            "nfm" => smol_commands::nfm::run(message.sender.id.as_str()).await,
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
