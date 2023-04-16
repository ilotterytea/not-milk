package kz.ilotterytea.fembajbot.api;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import org.reflections.Reflections;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.InvocationTargetException;
import java.util.HashSet;
import java.util.Optional;
import java.util.Set;

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

    public Optional<String> run(String id, IRCMessageEvent event, ParsedMessage message) {
        Optional<Command> command = getCommand(id);
        Optional<String> response = Optional.empty();

        if (command.isPresent()) {
            response = command.get().run(event, message);
        }

        return response;
    }

    public Optional<Command> getCommand(String id) {
        return this.COMMANDS.stream().filter(c -> c.getId().equals(id) || c.getAliases().contains(id)).findFirst();
    }
}
