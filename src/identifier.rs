use crate::options::Options;
use crate::regex_pd::PatternData;
use crate::Filter;
use anyhow::Context;
use fancy_regex::Regex;
use std::path::Path;
use std::fs;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Match {
    pub matched_on: String,
    pub name: String,
    pub rarity: f32,
    pub description: Option<String>,
    pub link: Option<String>,
    pub exploit: Option<String>,
}

pub fn identify_directory(path: &Path, regex: &Vec<PatternData>, matches: &mut Vec<Match>, filter: &Filter) -> anyhow::Result<()> {
    println!("Identifying directory: {:?}", path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            identify_file(&file_path, regex, matches, filter)?;
        } else if file_path.is_dir() {
            identify_directory(&file_path, &regex, matches, filter)?;
        }
    }
    Ok(())
}

pub fn identify_file(path: &Path, regex: &Vec<PatternData>, matches: &mut Vec<Match>, filter: &Filter) -> anyhow::Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    identify_text(content, regex, matches, filter);
    Ok(())
}

pub fn identify_text(text: String, regex_data: &Vec<PatternData>, matches: &mut Vec<Match>, filter: &Filter) {
    for r in regex_data {
        if r.rarity <= filter.min && r.rarity > filter.max {
            continue
        }
        let regex_pattern = match &r.regex_no_anchor {
            Some(pattern) if filter.borderless => pattern,
            _ => &r.regex
        };
        // Find all matches
        let re = Regex::new(&regex_pattern).unwrap();
        for mat in re.find_iter(&*text) {
            matches.push(
                Match {
                    matched_on: mat.unwrap().as_str().to_string(),
                    name: r.name.as_str().to_string(),
                    rarity: r.rarity,
                    description: match &r.description {
                        Some(description) => Some(description.to_string()),
                        None => None
                    },
                    link: match &r.url {
                        Some(url) => Some(url.to_string()),
                        None => None
                    },
                    exploit: match &r.exploit {
                        Some(exploit) => Some(exploit.to_string()),
                        None => None
                    },
                }
            );
        }
    }
}

pub fn identify(input: &String, regex_data: Vec<PatternData>, matches: &mut Vec<Match>, filter: &Filter, options: &Options) -> anyhow::Result<()> {
    let path = Path::new(input);
    if !options.only_text && path.exists() {
        if path.is_file() {
            identify_file(path, &regex_data, matches, &filter)?;
        } else if path.is_dir() {
            identify_directory(path, &regex_data, matches, &filter)?;
        } else {
            panic!("Input is path but neither file nor directory");
        }
    } else {
        identify_text(input.to_string(), &regex_data, matches, &filter);
    }

    Ok(())
}