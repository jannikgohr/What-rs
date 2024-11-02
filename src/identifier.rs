use crate::regex_pd::DATA;
use crate::Filter;
use fancy_regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Match {
    pub matched_on: String,
    pub name: String,
    pub rarity: f32,
    pub description: Option<String>,
    pub link: Option<String>,
    pub exploit: Option<String>,
}

pub fn identify_directory(path: &Path, matches: &mut Vec<Match>, filter: &Filter) -> anyhow::Result<()> {
    println!("Identifying directory: {:?}", path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            identify_file(&file_path, matches, filter)?;
        } else if file_path.is_dir() {
            identify_directory(&file_path, matches, filter)?;
        }
    }
    Ok(())
}

pub fn identify_file(path: &Path, matches: &mut Vec<Match>, filter: &Filter) -> anyhow::Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = read_file_to_strings(path).join("\n");
    identify_text(content, matches, filter);
    Ok(())
}

pub fn identify_text(text: String, matches: &mut Vec<Match>, filter: &Filter) {
    for r in DATA {
        if r.rarity <= filter.min && r.rarity > filter.max {
            continue
        }
        let regex_pattern = if filter.borderless {
            &r.regex_no_anchor
        } else {
            &r.regex
        };
        // Find all matches
        let re = Regex::new(&regex_pattern).unwrap();
        for mat in re.find_iter(&*text) {
            matches.push(
                Match {
                    matched_on: mat.unwrap().as_str().to_string(),
                    name: r.name.parse().unwrap(),
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

pub fn identify(input: &String, matches: &mut Vec<Match>, filter: &Filter, only_text: bool) -> anyhow::Result<()> {
    let path = Path::new(input);
    if !only_text && path.exists() {
        if path.is_file() {
            identify_file(path, matches, &filter)?;
        } else if path.is_dir() {
            identify_directory(path, matches, &filter)?;
        } else {
            panic!("Input is path but neither file nor directory");
        }
    } else {
        identify_text(input.to_string(), matches, &filter);
    }

    Ok(())
}

fn read_file_to_strings(filename: &Path) -> Vec<String> {
    let file = fs::read(filename).expect("File not found");

    let mut printable_text: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut use_current_buffer = false;

    //we only need the human-readable strings from the file.
    for character in file {
        if character.is_ascii_graphic() {
            // Doesn't consider whitespace as a graphic!
            use_current_buffer = true;
            buffer.push(character);
        } else if use_current_buffer {
            // If the char isn't ascii graphic, that means this is the end for our string which we are interested in
            // string with length less than 4 most likely won't be of our use.
            // If it has length more than 4, then push it to our `printable_text`
            if buffer.len() >= 4 {
                printable_text.push(
                    String::from_utf8(buffer.clone()).expect("failed to convert u8 to string"),
                );
            }

            // Clear the buffer so that current contents of it won't affect the next string.
            buffer.clear();
            // We set this to false because we don't want to use buffer until we get a ascii graphic!
            use_current_buffer = false;
        }
    }

    printable_text.push(String::from_utf8(buffer).expect("failed to convert u8 to string"));

    printable_text
}