package kz.ilotterytea.fembajbot.builtin;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import kz.ilotterytea.fembajbot.TwitchBot;
import kz.ilotterytea.fembajbot.api.Command;
import kz.ilotterytea.fembajbot.api.ParsedMessage;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.entities.Consumer;
import kz.ilotterytea.fembajbot.utils.HibernateUtil;
import org.hibernate.Session;

import java.util.Collections;
import java.util.List;
import java.util.Optional;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class JoinCommand implements Command {
    @Override
    public String getId() {
        return "join";
    }

    @Override
    public int getDelay() {
        return 0;
    }

    @Override
    public List<String> getSubcommands() {
        return Collections.emptyList();
    }

    @Override
    public List<String> getAliases() {
        return Collections.emptyList();
    }

    @Override
    public Optional<String> run(IRCMessageEvent event, ParsedMessage message, Consumer consumer, Channel channel) {
        // Checking for the existence of this channel in the database
        Session session = HibernateUtil.getSessionFactory().openSession();
        List<Channel> channels = session.createQuery("from Channel where aliasId = :aliasId", Channel.class)
                .setParameter("aliasId", event.getUser().getId())
                .getResultList();

        if (!channels.isEmpty()) {
            return Optional.of(String.format("%s: sorry, i'm already supplying \"not milk\" in this chat room! \uD83D\uDE21 ", event.getUser().getName()));
        }

        // Creating a new channel
        Channel targetChannel = new Channel(Integer.parseInt(event.getUser().getId()));

        session.getTransaction().begin();
        session.persist(targetChannel);
        session.getTransaction().commit();
        session.close();

        // Success announcement
        TwitchBot.getInstance().getClient().getChat().joinChannel(event.getUser().getName());
        TwitchBot.getInstance().getClient().getChat().sendMessage(
                event.getUser().getName(),
                String.format("hiii %s! i will be supplying \"not milk\" to your chat! \uD83E\uDD5B \uD83D\uDCE6 \uD83D\uDE0A ", event.getUser().getName())
        );

        return Optional.of(String.format("%s: successfully joined your chat!", event.getUser().getName()));
    }
}
