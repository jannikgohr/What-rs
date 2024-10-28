use std::fs;
use std::path::Path;
use anyhow::Context;
use fancy_regex::Regex;
use crate::Filter;
use crate::regex_pd::PatternData;

pub fn identify_directory(path: &Path, regex: &Vec<PatternData>, filter: &Filter) -> anyhow::Result<()> {
    println!("Identifying directory: {:?}", path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            identify_file(&file_path, regex, filter)?;
        } else if file_path.is_dir() {
            identify_directory(&file_path, &regex, filter)?;
        }
    }
    Ok(())
}

pub fn identify_file(path: &Path, regex: &Vec<PatternData>, filter: &Filter) -> anyhow::Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    identify_text(content, regex, filter);
    Ok(())
}

pub fn identify_text(text: String, regex_data: &Vec<PatternData>, filter: &Filter) {
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
            println!();
            println!("Found match: {}", mat.unwrap().as_str());
            println!("Type: {}", r.name.as_str());
            println!("Using: {}", regex_pattern);
        }
    }
}

