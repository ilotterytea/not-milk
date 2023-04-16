package kz.ilotterytea.fembajbot.api;

import kz.ilotterytea.fembajbot.SharedConstants;
import kz.ilotterytea.fembajbot.TwitchBot;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Optional;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class ParsedMessage {
    private final String id;
    private final String subcommand;
    private final String message;

    public ParsedMessage(String id, String subcommand, String message) {
        this.id = id;
        this.subcommand = subcommand;
        this.message = message;
    }

    public static Optional<ParsedMessage> parse(String rawMessage) {
        ArrayList<String> s = null;
        for (String prefix : SharedConstants.DEFAULT_PREFIXES) {
            if (rawMessage.startsWith(prefix)) {
                s = new ArrayList<>(Arrays.asList(rawMessage.substring(prefix.length()).trim().split(" ")));
                break;
            }
        }

        if (s == null) {
            return Optional.empty();
        }

        String id = s.get(0);
        Optional<Command> command = TwitchBot.getInstance().getCommandLoader().getCommand(id);

        if (command.isEmpty()) {
            return Optional.empty();
        }

        s.remove(0);

        String subcommand = null;

        if (!s.isEmpty() && command.get().getSubcommands().contains(s.get(0))) {
            subcommand = s.get(0);
            s.remove(0);
        }

        return Optional.of(new ParsedMessage(
                id,
                subcommand,
                String.join(" ", s)
        ));
    }

    public String getId() {
        return id;
    }

    public String getSubcommand() {
        return subcommand;
    }

    public String getMessage() {
        return message;
    }
}
