package kz.ilotterytea.fembajbot.schemas;

import com.google.gson.annotations.SerializedName;

import java.util.List;

/**
 * @author ilotterytea
 * @version 1.0
 */
public class GlobalLines {
    @SerializedName("legendary_lines")
    private List<String> legendaryLines;
    @SerializedName("epic_lines")
    private List<String> epicLines;
    @SerializedName("common_lines")
    private List<String> commonLines;
    @SerializedName("poor_lines")
    private List<String> poorLines;
    @SerializedName("nothing_found_lines")
    private List<String> nothingFoundLines;

    public GlobalLines() {}

    public List<String> getLegendaryLines() {
        return legendaryLines;
    }

    public List<String> getEpicLines() {
        return epicLines;
    }

    public List<String> getCommonLines() {
        return commonLines;
    }

    public List<String> getPoorLines() {
        return poorLines;
    }

    public List<String> getNothingFoundLines() {
        return nothingFoundLines;
    }
}
