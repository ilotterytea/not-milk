package kz.ilotterytea.fembajbot.builtin;

import com.github.twitch4j.chat.events.channel.IRCMessageEvent;
import kz.ilotterytea.fembajbot.TwitchBot;
import kz.ilotterytea.fembajbot.api.Command;
import kz.ilotterytea.fembajbot.api.ParsedMessage;
import kz.ilotterytea.fembajbot.entities.Channel;
import kz.ilotterytea.fembajbot.entities.Consumer;
import kz.ilotterytea.fembajbot.schemas.GlobalLines;
import kz.ilotterytea.fembajbot.utils.HibernateUtil;
import kz.ilotterytea.fembajbot.utils.MathUtils;
import org.hibernate.Session;

import java.util.*;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class SipCommand implements Command {
    @Override
    public String getId() {
        return "sip";
    }

    @Override
    public int getDelay() {
        return 1800000;
    }

    @Override
    public List<String> getSubcommands() {
        return Collections.emptyList();
    }

    @Override
    public List<String> getAliases() {
        return Arrays.asList("drink", "yum");
    }

    @Override
    public Optional<String> run(IRCMessageEvent event, ParsedMessage message, Consumer consumer, Channel channel) {
        // Channel:
        Session session = HibernateUtil.getSessionFactory().openSession();
        List<Consumer> consumers = session.createQuery("from Consumer", Consumer.class).getResultList();

        int position = consumers.indexOf(consumer);

        if (position >= 0) {
            consumers.remove(position);
        }

        // Random points
        final int MIN_POINTS = -30;
        final int MAX_POINTS = 30;

        final GlobalLines lines = TwitchBot.getInstance().getGlobalLines();

        int funChance = MathUtils.getRandomNumber(0, 100);
        final String msg;
        final int points;

        session.getTransaction().begin();

        // 'Nothing found' clause:
        if (MathUtils.isBetween(funChance, 0, 10)) {
            points = 0;

            if (lines != null) {
                msg = lines.getNothingFoundLines().get(MathUtils.getRandomNumber(0, lines.getNothingFoundLines().size() - 1));
            } else {
                msg = "missingno.";
            }
        }
        // 'Negative' clause:
        else if (MathUtils.isBetween(funChance, 11, 30)) {
            points = MathUtils.getRandomNumber(MIN_POINTS, -1);

            if (lines != null) {
                msg = lines.getPoorLines().get(MathUtils.getRandomNumber(0, lines.getPoorLines().size() - 1));
            } else {
                msg = "missingno.";
            }
        }
        // 'Steal from someone' clause:
        else if (MathUtils.isBetween(funChance, 31, 50)) {
            points = MathUtils.getRandomNumber(1, MAX_POINTS);

            Consumer randomConsumer = consumers.get(MathUtils.getRandomNumber(0, consumers.size() - 1));

            randomConsumer.setPoints(randomConsumer.getPoints() - points);
            session.persist(randomConsumer);

            String rudeness;
            String action;
            int percent = (points / MAX_POINTS) * 100;

            if (MathUtils.isBetween(percent, 80, 100)) {
                rudeness = "rudely";
                action = "made him about to milk \uD83E\uDD5B \uD83D\uDE2B";
            } else if (MathUtils.isBetween(percent, 59, 79)) {
                rudeness = "gently";
                action = "asked them to pour you some milk \uD83E\uDD5B \uD83E\uDD13";
            } else {
                rudeness = "";
                action = "he's already got a little milk dripping out of him \uD83E\uDD5B \uD83E\uDD74";
            }


            msg = String.format(
                    "you lacked milk, but you %s took %s and %s",
                    rudeness,
                    randomConsumer.getAliasName(),
                    action
            );
        }
        // 'Positive' clause:
        else {
            points = MathUtils.getRandomNumber(1, MAX_POINTS);

            if (lines != null) {
                int percent = (points / MAX_POINTS) * 100;

                if (MathUtils.isBetween(percent, 70, 100)) {
                    msg = lines.getLegendaryLines().get(MathUtils.getRandomNumber(0, lines.getLegendaryLines().size() - 1));
                } else if (MathUtils.isBetween(percent, 39, 69)) {
                    msg = lines.getEpicLines().get(MathUtils.getRandomNumber(0, lines.getEpicLines().size() - 1));
                } else {
                    msg = lines.getCommonLines().get(MathUtils.getRandomNumber(0, lines.getCommonLines().size() - 1));
                }
            } else {
                msg = "missingno.";
            }
        }

        // Updating:
        consumer.setPoints(consumer.getPoints() + points);

        session.persist(consumer);
        session.getTransaction().commit();

        session.close();

        return Optional.of(String.format(
                "%s: %s ... anyways you got %s | total: %s \uD83E\uDD5B ",
                event.getUser().getName(),
                msg,
                (points >= 0) ? "+" + points : "-" + points,
                consumer.getPoints()
        ));
    }
}
