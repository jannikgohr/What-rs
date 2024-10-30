use fancy_regex::Regex;
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
pub struct PatternData {
    pub name: String,
    pub regex: String,
    pub regex_no_anchor : Option<String>,
    // pub plural_name: bool,
    pub description: Option<String>,
    pub exploit: Option<String>,
    pub rarity: f32,
    pub url: Option<String>,
    // pub tags: Option<Vec<String>>,
    // pub children: Option<ChildrenData>,
}

/*
TODO: Use this data to give users more specific data
#[derive(Debug, Deserialize)]
pub struct ChildrenData {
    pub path: String,
    pub entry: String,
    pub method: String,
    pub deletion_pattern: Option<String>,
}
 */


pub fn load_regex_pattern_data(json_string: &str) -> anyhow::Result<Vec<PatternData>, io::Error> {

    let mut json_data: Vec<PatternData> = serde_json::from_str(&json_string)?;

    for pattern in &mut json_data {
        // Regex to remove `^` not within `[]` or escaped
        let re_start = Regex::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])").unwrap();
        // Regex to remove `$` not within `[]` or escaped
        let re_end = Regex::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])").unwrap();

        // Apply the regex replacements
        let regex_no_start_anchor = re_start.replace_all(&pattern.regex, "");
        let regex_no_anchor = re_end.replace_all(&regex_no_start_anchor, "");
        pattern.regex_no_anchor = Option::from(regex_no_anchor.to_string());
    }

    Ok(json_data)
}