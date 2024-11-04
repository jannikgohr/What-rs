use serde::Serialize;
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct PatternData {
    pub name: &'static str,
    //pub regex: &'static str,
    //#[serde(skip_deserializing)]
    //pub regex_no_anchor: &'static str,
    pub plural_name: bool,
    pub description: Option<&'static str>,
    pub exploit: Option<&'static str>,
    pub rarity: f32,
    pub url: Option<&'static str>,
    pub tags: &'static [&'static str],
    // pub uses_non_standard_regex: bool, TODO: use it
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

const _1: [PatternData; 0] = []; // so rust-analyzer won't complain about unused imports
// this is `pub const DATA: [PatternData; 129] = ...`
include!(concat!(env!("OUT_DIR"), "/data.rs"));

const _2: [Lazy<Regex>; 0] = []; // so rust-analyzer won't complain about unused imports
// this is `pub static REGEX: [Lazy<Regex>; 129] = ...`
// this is `pub static REGEX_NO_ANCHOR: [Lazy<Regex>; 129] = ...`
include!(concat!(env!("OUT_DIR"), "/regex_data.rs"));

