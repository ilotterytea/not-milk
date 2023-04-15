package kz.ilotterytea.fembajbot;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class TwitchBot {
    private final Logger LOGGER = LoggerFactory.getLogger(TwitchBot.class.getName());

    private OAuth2Credential credential;
    private TwitchClient client;

    private static TwitchBot instance;

    public OAuth2Credential getCredential() {
        return credential;
    }

    public TwitchClient getClient() {
        return client;
    }

    public static TwitchBot getInstance() {
        return instance;
    }

    public TwitchBot() {
        instance = this;
    }

    public void run() {
        credential = new OAuth2Credential("twitch", SharedConstants.TWITCH_OAUTH2_TOKEN);

        client = TwitchClientBuilder.builder()
                .withChatAccount(credential)
                .withEnableHelix(true)
                .withEnableChat(true)
                .build();

        client.getChat().connect();

        if (credential.getUserName() != null) {
            client.getChat().joinChannel(credential.getUserName());
        }

        client.getEventManager().onEvent(IRCMessageEvent.class, event -> {
            LOGGER.debug(event.toString());
        });
    }
}
