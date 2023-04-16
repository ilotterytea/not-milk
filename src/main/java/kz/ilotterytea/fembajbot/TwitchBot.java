package kz.ilotterytea.fembajbot;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import com.github.twitch4j.helix.domain.User;
import kz.ilotterytea.fembajbot.api.CommandLoader;
import kz.ilotterytea.fembajbot.api.ParsedMessage;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.utils.HibernateUtil;
import org.hibernate.Session;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class TwitchBot {
    private final Logger LOGGER = LoggerFactory.getLogger(TwitchBot.class.getName());

    private OAuth2Credential credential;
    private TwitchClient client;
    private CommandLoader commandLoader;

    private static TwitchBot instance;

    public OAuth2Credential getCredential() {
        return credential;
    }

    public TwitchClient getClient() {
        return client;
    }

    public CommandLoader getCommandLoader() {
        return commandLoader;
    }

    public static TwitchBot getInstance() {
        return instance;
    }

    public TwitchBot() {
        instance = this;
    }

    public void run() {
        commandLoader = new CommandLoader();

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

        // Joining "channel" entities chats:
        Session session = HibernateUtil.getSessionFactory().openSession();
        List<Channel> channels = session.createQuery("from Channel where optOutTimestamp is null", Channel.class).getResultList();

        if (!channels.isEmpty()) {
            List<User> convertedChannels = client.getHelix().getUsers(
                    null,
                    channels.stream().map(c -> c.getAliasId().toString()).collect(Collectors.toList()),
                    null
            ).execute().getUsers();

            for (User channel : convertedChannels) {
                client.getChat().joinChannel(channel.getLogin());
                LOGGER.debug("Joined " + channel.getLogin() + "'s chatroom!");
            }
        }

        session.close();

        client.getEventManager().onEvent(IRCMessageEvent.class, event -> {
            LOGGER.debug(event.toString());

            if (event.getMessage().isEmpty()) {
                return;
            }

            Optional<ParsedMessage> message = ParsedMessage.parse(event.getMessage().get());

            if (message.isPresent()) {
                Optional<String> response = commandLoader.run(message.get().getId(), event, message.get());

                response.ifPresent(s -> client.getChat().sendMessage(event.getChannel().getName(), s));
            }
        });
    }
}
