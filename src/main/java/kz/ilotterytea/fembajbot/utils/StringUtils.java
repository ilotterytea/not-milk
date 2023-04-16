package kz.ilotterytea.fembajbot.utils;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class StringUtils {
    public static String formatTimestamp(long timestamp) {
        long d = Math.round(timestamp / (60 * 60 * 24));
        long h = Math.round(timestamp / (60 * 60) % 24);
        long m = Math.round(timestamp % (60 * 60) / 60);
        long s = Math.round(timestamp % 60);

        // Only seconds:
        if (d == 0 && h == 0 && m == 0) {
            return String.format("%ss", s);
        }
        // Minutes and seconds:
        else if (d == 0 && h == 0) {
            return String.format("%sm%ss", m, s);
        }
        // Hours and minutes:
        else if (d == 0) {
            return String.format("%sh%sm", h, m);
        }
        // Days and hours:
        else {
            return String.format("%sd%sh", d, h);
        }
    }
}
