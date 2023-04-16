package kz.ilotterytea.fembajbot.api;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;

import java.util.List;
import java.util.Optional;

/**
 * @author ilotterytea
 * @version 1.0
 */
public interface Command {
    String getId();
    int getDelay();
    List<String> getSubcommands();
    List<String> getAliases();

    Optional<String> run(IRCMessageEvent event, ParsedMessage message);
}