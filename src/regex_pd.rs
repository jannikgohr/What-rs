use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct PatternData {
    pub name: &'static str,
    pub regex: &'static str,
    #[serde(skip_deserializing)]
    pub regex_no_anchor: &'static str,
    pub plural_name: bool,
    pub description: Option<&'static str>,
    pub exploit: Option<&'static str>,
    pub rarity: f32,
    pub url: Option<&'static str>,
    pub tags: &'static [&'static str],
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

// this is `pub const DATA: [PatternData; 129] = ...`
include!(concat!(env!("OUT_DIR"), "/data.rs"));

