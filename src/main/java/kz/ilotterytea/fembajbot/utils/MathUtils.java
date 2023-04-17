package kz.ilotterytea.fembajbot.utils;

import java.util.Random;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class MathUtils {
    public static int getRandomNumber(int min, int max) {
        return new Random().nextInt((max - min) + 1) + min;
    }

    public static boolean isBetween(int number, int min, int max) {
        return number >= min && number <= max;
    }
}
