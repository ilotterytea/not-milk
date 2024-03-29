use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

use crate::{
    smol_commands,
    utils::{sync_consumer, ParsedMessage},
};

pub async fn irc_message_handler(
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    message: PrivmsgMessage,
) {
    let text = &message.message_text;

    let _msg_args = ParsedMessage::parse(text.as_str(), '🥛');

    if _msg_args.is_some() {
        let msg_args = _msg_args.as_ref().unwrap();

        let consumer = sync_consumer(message.sender.id.as_str()).await.unwrap();

        let msg: Option<String> = match msg_args.command_id.as_str() {
            "sip" => smol_commands::sip::run(consumer, msg_args),
            "place" => smol_commands::place::run(consumer, msg_args).await,
            "top" => smol_commands::top::run(consumer, msg_args),
            "join" => smol_commands::join::run(consumer, msg_args, client).await,
            "nfm" => smol_commands::nfm::run(consumer, msg_args).await,
            "yoink" | "suck" | "pump" => smol_commands::yoink::run(consumer, msg_args).await,
            "sellnfm" => smol_commands::sellnfm::run(consumer, msg_args),
            "give" => smol_commands::give::run(consumer, msg_args),
            "roulette" => smol_commands::roulette::run(consumer, msg_args),
            "mynfms" => smol_commands::mynfms::run(consumer, msg_args),
            "nfminfo" => smol_commands::nfminfo::run(consumer, msg_args),
            _ => None,
        };

        if msg.is_some() {
            client
                .say(message.channel_login.to_owned(), msg.unwrap())
                .await
                .expect("Unable to send a message");
        }
    }
}
