package kz.ilotterytea.fembajbot;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.FileInputStream;
import java.io.IOException;
import java.util.Properties;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class SharedConstants {
    public static final String TWITCH_OAUTH2_TOKEN;
    public static final String[] DEFAULT_PREFIXES;
    public static final Integer SIP_MAX;
    public static final Integer SIP_MIN;

    static {
        Properties properties = new Properties();

        Logger logger = LoggerFactory.getLogger(SharedConstants.class.getName());

        try (FileInputStream fis = new FileInputStream(System.getProperty("config_file", "config.properties"))) {
            properties.load(fis);
        } catch (IOException e) {
            logger.error("Couldn't load the properties file: " + e.getMessage());
        }

        TWITCH_OAUTH2_TOKEN = properties.getProperty("twitch.oauth2_token", null);
        DEFAULT_PREFIXES = properties.getProperty("default_prefixes", "~,\uD83E\uDD5B").split(",");
        SIP_MAX = Integer.parseInt(properties.getProperty("sip.max", "20"));
        SIP_MIN = Integer.parseInt(properties.getProperty("sip.max", "-20"));
    }
}
