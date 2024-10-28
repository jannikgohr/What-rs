use std::fs::File;
use std::io;
use std::io::Read;
use fancy_regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PatternData {
    pub name: String,
    pub regex: String,
    pub regex_no_anchor : Option<String>,
    pub plural_name: bool,
    pub description: Option<String>,
    pub exploit: Option<String>,
    pub rarity: f64,
    pub url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub children: Option<ChildrenData>,
    pub examples: Option<ExamplesData>,
}

#[derive(Debug, Deserialize)]
pub struct ChildrenData {
    pub path: String,
    pub entry: String,
    pub method: String,
    pub deletion_pattern: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExamplesData {
    pub valid: Option<Vec<String>>,
    pub invalid: Option<Vec<String>>,
}

pub fn load_regex_pattern_data(file_path: &str) -> anyhow::Result<Vec<PatternData>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a Vec<DataEntry>
    let mut json_data: Vec<PatternData> = serde_json::from_str(&contents)?;


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