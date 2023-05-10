package kz.ilotterytea.fembajbot;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import com.github.twitch4j.helix.domain.User;
import com.google.gson.Gson;
import kz.ilotterytea.fembajbot.api.CommandLoader;
import kz.ilotterytea.fembajbot.api.ParsedMessage;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.entities.Consumer;
import kz.ilotterytea.fembajbot.schemas.GlobalLines;
import kz.ilotterytea.fembajbot.utils.HibernateUtil;
import kz.ilotterytea.fembajbot.utils.ResourceUtils;
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
    private GlobalLines globalLines;

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

    public GlobalLines getGlobalLines() {
        return globalLines;
    }

    public static TwitchBot getInstance() {
        return instance;
    }

    public TwitchBot() {
        instance = this;
    }

    public void run() {
        globalLines = new Gson().fromJson(ResourceUtils.readFileFromResources("lines/lines_en.json"), GlobalLines.class);

        commandLoader = new CommandLoader();

        credential = new OAuth2Credential("twitch", SharedConstants.TWITCH_OAUTH2_TOKEN);

        client = TwitchClientBuilder.builder()
                .withChatAccount(credential)
                .withEnableHelix(true)
                .withEnableChat(true)
                .build();

        client.getChat().connect();

        Session session = HibernateUtil.getSessionFactory().openSession();

        if (credential.getUserName() != null) {
            client.getChat().joinChannel(credential.getUserName());

            // Create a channel in the database for the bot if it does not exist:
            if (
                    session.createQuery("from Channel where aliasId = :aliasId", Channel.class)
                            .setParameter("aliasId", credential.getUserId())
                            .getResultList()
                            .isEmpty()
            ) {
                Channel channel = new Channel(Integer.parseInt(credential.getUserId()));

                session.getTransaction().begin();
                session.persist(channel);
                session.getTransaction().commit();
            }
        }

        // Joining "channel" entities chats:
        List<Channel> channels = session.createQuery("from Channel where optOutTimestamp is null", Channel.class).getResultList();

        if (!channels.isEmpty()) {
            List<User> convertedChannels = client.getHelix().getUsers(
                    credential.getAccessToken(),
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
                Session session1 = HibernateUtil.getSessionFactory().openSession();
                session1.getTransaction().begin();

                // Getting the current channel:
                List<Channel> eventChannels = session1.createQuery("from Channel where aliasId = :aliasId", Channel.class)
                        .setParameter("aliasId", event.getChannel().getId())
                        .getResultList();

                Channel eventChannel;

                if (eventChannels.isEmpty()) {
                    eventChannel = new Channel(Integer.parseInt(event.getChannel().getId()));

                    session1.persist(eventChannel);
                    session1.getTransaction().commit();
                } else {
                    eventChannel = eventChannels.get(0);
                }

                // Create a new consumer if it does not exist:
                List<Consumer> consumers = session1.createQuery("from Consumer where aliasId = :aliasId", Consumer.class)
                        .setParameter("aliasId", event.getUser().getId())
                        .getResultList();

                Consumer consumer;

                if (consumers.isEmpty()) {
                    consumer = new Consumer(Integer.parseInt(event.getUser().getId()), event.getUser().getName());

                    session1.persist(consumer);
                    session1.getTransaction().commit();
                } else {
                    consumer = consumers.get(0);
                }

                session1.close();

                // Run the command:
                Optional<String> response = commandLoader.run(message.get().getId(), event, message.get(), consumer, eventChannel);

                response.ifPresent(s -> client.getChat().sendMessage(event.getChannel().getName(), s));
            }
        });
    }
}
