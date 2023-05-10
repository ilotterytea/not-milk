package kz.ilotterytea.fembajbot.builtin;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import kz.ilotterytea.fembajbot.TwitchBot;
import kz.ilotterytea.fembajbot.api.Command;
import kz.ilotterytea.fembajbot.api.ParsedMessage;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.entities.Consumer;
import kz.ilotterytea.fembajbot.utils.StringUtils;

import java.lang.management.ManagementFactory;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Optional;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class PingCommand implements Command {
    @Override
    public String getId() {
        return "ping";
    }

    @Override
    public int getDelay() {
        return 5000;
    }

    @Override
    public List<String> getSubcommands() {
        return Collections.emptyList();
    }

    @Override
    public List<String> getAliases() {
        return Arrays.asList("pong", "tap", "health", "пинг");
    }

    @Override
    public Optional<String> run(IRCMessageEvent event, ParsedMessage message, Consumer consumer, Channel channel) {
        Runtime rt = Runtime.getRuntime();
        double usedMemMb = ((rt.totalMemory() - rt.freeMemory()) / 1024.0) / 1024.0;
        double totalMemMb = (rt.totalMemory() / 1024.0) / 1024.0;
        double percentMemUsage = Math.round((usedMemMb / totalMemMb) * 100.0);
        long latency = TwitchBot.getInstance().getClient().getChat().getLatency();

        return Optional.of(String.format(
                "%s: BrorDOVEintoMilk \uD83E\uDD5B Java %s \u00B7 Uptime: %s \u00B7 Memory usage: %s%% (%sMB) of %sMB \u00B7 TMI: %sms",
                event.getUser().getName(),
                System.getProperty("java.version", "N/A"),
                StringUtils.formatTimestamp(ManagementFactory.getRuntimeMXBean().getUptime() / 1000),
                Math.round(percentMemUsage),
                Math.round(usedMemMb),
                Math.round(totalMemMb),
                (latency < 0) ? "N/A" : latency
        ));
    }
}
