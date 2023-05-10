package kz.ilotterytea.fembajbot.api;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import kz.ilotterytea.fembajbot.entities.Action;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.entities.Consumer;
import kz.ilotterytea.fembajbot.utils.HibernateUtil;
import org.hibernate.Session;
import org.reflections.Reflections;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.InvocationTargetException;
import java.util.*;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class CommandLoader extends ClassLoader {
    private final Set<Command> COMMANDS;
    private final Logger LOGGER = LoggerFactory.getLogger(CommandLoader.class);

    public CommandLoader() {
        super();
        COMMANDS = new HashSet<>();
        init();
    }

    private void init() {
        Reflections reflections = new Reflections("kz.ilotterytea.fembajbot.builtin");

        Set<Class<? extends Command>> classes = reflections.getSubTypesOf(Command.class);

        for (Class<? extends Command> clazz : classes) {
            try {
                Command command = clazz.getDeclaredConstructor().newInstance();
                COMMANDS.add(command);
                LOGGER.debug("Successfully loaded the '" + command.getId() + "' command!");
            } catch (InstantiationException | IllegalAccessException | InvocationTargetException |
                     NoSuchMethodException e) {
                throw new RuntimeException(e);
            }
        }
    }

    public Optional<String> run(String id, IRCMessageEvent event, ParsedMessage message, Consumer consumer, Channel channel) {
        Optional<Command> command = getCommand(id);

        if (command.isEmpty()) {
            return Optional.empty();
        }

        Session session = HibernateUtil.getSessionFactory().openSession();

        List<Action> actions = session
                .createQuery(
                        "from Action where consumer = :consumer AND channel = :channel AND commandId = :commandId ORDER BY creationTimestamp DESC",
                        Action.class
                )
                .setParameter("consumer", consumer)
                .setParameter("channel", channel)
                .setParameter("commandId", id)
                .getResultList();

        if (!actions.isEmpty()) {
            Action action = actions.get(0);

            if (new Date().getTime() - action.getCreationTimestamp().getTime() < command.get().getDelay()) {
                return Optional.empty();
            }
        }

        Action action = new Action(channel, consumer, id, ((message.getSubcommand() != null) ? message.getSubcommand() + " " : "") + message.getMessage());
        channel.addAction(action);
        consumer.addAction(action);

        session.getTransaction().begin();
        session.merge(channel);
        session.merge(consumer);
        session.persist(action);
        session.getTransaction().commit();

        session.close();

        return command.get().run(event, message, consumer, channel);
    }

    public Optional<Command> getCommand(String id) {
        return this.COMMANDS.stream().filter(c -> c.getId().equals(id) || c.getAliases().contains(id)).findFirst();
    }
}
