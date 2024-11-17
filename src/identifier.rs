mod pcap;

use std::collections::HashSet;
use crate::regex_pd::{PATTERN_DATA, REGEX, REGEX_NO_ANCHOR};
use crate::Filter;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use crate::identifier::pcap::identify_pcapng;
use crate::options::Options;

#[derive(Debug, Serialize)]
pub struct Match {
    pub matched_on: String,
    pub name: String,
    pub rarity: f32,
    pub description: Option<String>,
    pub link: Option<String>,
    pub exploit: Option<String>,
}

pub struct Identifier {
    matched_texts: Arc<RwLock<HashSet<String>>>,
}

impl Identifier {

    pub fn new() -> Identifier {
        Identifier {
            matched_texts: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn identify_text(&mut self, text: String, matches: &mut Vec<Match>, filter: &Filter, options: &Options) {
        let text = Arc::new(text);
        let matches_arc = Arc::new(Mutex::new(Vec::new()));

        PATTERN_DATA
            .par_iter()
            .enumerate()
            .for_each(|(i, r)| {
                if filter.gets_excluded(&r) {
                    return;
                }

                let re: &Lazy<Regex> = if filter.borderless {
                    &REGEX_NO_ANCHOR[i]
                } else {
                    &REGEX[i]
                };

                // Find all matches for this pattern
                for mat in re.find_iter(&text) {

                    let matched_on = mat.as_str().to_string();

                    if !options.allow_duplicates {
                        if self.matched_texts.read().unwrap().contains(&matched_on) {
                            continue
                        } else {
                            self.matched_texts.write().unwrap().insert(matched_on.clone());
                        }
                    }

                    let match_obj = Match {
                        matched_on,
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
                    };

                    // Push the match object to the shared vector
                    let mut matches_lock = matches_arc.lock().unwrap();
                    matches_lock.push(match_obj);
                }
            });

        // Move collected matches from matches_arc to the output vector
        let results = Arc::try_unwrap(matches_arc)
            .expect("Failed to unwrap Arc") // We ensure no other thread is holding a reference
            .into_inner()
            .expect("Failed to lock Mutex");

        matches.extend(results);
    }
}

pub fn identify_directory(path: &Path, matches: &mut Vec<Match>, filter: &Filter, options: &Options) -> anyhow::Result<()> {
    println!("Identifying directory: {:?}", path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            identify_file(&file_path, matches, filter, options)?;
        } else if file_path.is_dir() {
            identify_directory(&file_path, matches, filter, options)?;
        }
    }
    Ok(())
}

pub fn identify_file(path: &Path, matches: &mut Vec<Match>, filter: &Filter, options: &Options) -> anyhow::Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);

    if options.pcapng {
        identify_pcapng(path, matches, filter, options)?;
    } else {
        let content = read_file_to_strings(path).join("\n");
        Identifier::new().identify_text(content, matches, filter, options);
    }

    Ok(())
}

pub fn identify(input: &String, matches: &mut Vec<Match>, filter: &Filter, options: &Options) -> anyhow::Result<()> {
    let path = Path::new(input);
    if !options.only_text && path.exists() {
        if path.is_file() {
            identify_file(path, matches, &filter, &options)?;
        } else if path.is_dir() {
            identify_directory(path, matches, &filter, &options)?;
        } else {
            panic!("Input is path but neither file nor directory");
        }
    } else {
        Identifier::new().identify_text(input.to_string(), matches, &filter, &options);
    }

    Ok(())
}

fn read_file_to_strings(filename: &Path) -> Vec<String> {
    let file = fs::read(filename).expect("File not found");
    to_human_readable_vec(file)
}

pub(crate) fn to_human_readable_vec(b_string: Vec<u8>) -> Vec<String> {
    let mut printable_text: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut use_current_buffer = false;

    for character in b_string {
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