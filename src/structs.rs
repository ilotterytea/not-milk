use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lines {
    pub legendary_lines: Vec<String>,
    pub epic_lines: Vec<String>,
    pub common_lines: Vec<String>,
    pub poor_lines: Vec<String>,
}
