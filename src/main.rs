extern crate regex;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{fs, io, process};

#[derive(Debug, Deserialize)]
struct DataEntry {
    name: String,
    regex: String,
    plural_name: bool,
    description: Option<String>,
    rarity: f64,
    url: Option<String>,
    tags: Option<Vec<String>>,
    children: Option<ChildrenData>,
    examples: Option<ExamplesData>,
}

#[derive(Debug, Deserialize)]
struct ChildrenData {
    path: String,
    entry: String,
    method: String,
    deletion_pattern: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExamplesData {
    valid: Option<Vec<String>>,
    invalid: Option<Vec<String>>,
}

struct Filter {
    min: f64,
    max: f64,
}

fn main() -> Result<()> {
    let matches = Command::new("pyWhat in Rust")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jannik Gohr <https://github.com/jannikgohr/What-rs>")
        .about("Identify what something is.")
        .arg(
            Arg::new("text_input")
                .help("Text input to identify")
                .required(false),
        )
        .arg(
            Arg::new("tags")
                .short('t')
                .long("tags")
                .help("Show available tags and exit.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("rarity")
                .short('r')
                .long("rarity")
                .default_value("0.1:1")
                .help("Filter by rarity, range of 0:1."),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .help("Only show matches with these tags."),
        )
        .arg(
            Arg::new("exclude")
                .short('e')
                .long("exclude")
                .help("Exclude matches with these tags."),
        )
        .arg(
            Arg::new("only_text")
                .short('o')
                .long("only-text")
                .help("Do not scan files or folders.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("tags") {
        print_tags()?;
        process::exit(0);
    }

    let regex_data = load_regex_pattern_data("data/regex.json")?;
    // println!("{:#?}", regex.get(0));

    let rarity = matches
        .get_one::<String>("rarity")
        .map(|s| parse_rarity(s))
        .transpose()
        .context("Invalid rarity range format. Expected 'min:max'")?;

    let filter: Filter = create_filter(
        rarity,
        matches.get_one::<String>("include"),
        matches.get_one::<String>("exclude"),
    );

    let text_input = matches.get_one::<String>("text_input").cloned();
    if text_input.is_none() {
        eprintln!("Text input expected. Run '--help' for usage.");
        process::exit(1);
    }

    // Determine if the input is text or a file/directory path
    if let Some(input) = matches.get_one::<String>("text_input") {
        let path = Path::new(input);
        if !matches.get_flag("only_text") && path.exists(){
            // Handle as a file or directory path
            if path.is_file() {
                identify_file(path, &regex_data, &filter)?;
            } else if path.is_dir() {
                identify_directory(path, &regex_data, &filter)?;
            } else {
                eprintln!("Input path is not a file or directory");
                process::exit(1);
            }
        } else {
            // Handle as plain text
            identify_text(input.to_string(), &regex_data, &filter);
        }
    } else {
        eprintln!("Text input or file/directory path expected. Run '--help' for usage.");
        process::exit(1);
    }

    Ok(())
}

fn print_tags() -> Result<()> {
    println!("Available Tags:");
    // Code to retrieve and print available tags goes here
    Ok(())
}

fn parse_rarity(rarity: &str) -> Result<(f64, f64)> {
    let parts: Vec<&str> = rarity.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid rarity format. \
        Format must be 'min:max', where min and max are decimal numbers seperated by a colon.");
    }
    let min = parts[0].parse::<f64>()?;
    let max = parts[1].parse::<f64>()?;
    if min < 0f64 || max > 1f64 {
        anyhow::bail!("Invalid rarity range. Range must be between 0 and 1 inclusive.");
    }
    Ok((min, max))
}

fn create_filter(
    rarity: Option<(f64, f64)>,
    include: Option<&String>,
    exclude: Option<&String>,
) -> Filter {
    // TODO: Add include and exclude filter
    let mut filter: Filter = Filter { min: 0f64, max: 1f64 };
    
    if let Some((min, max)) = rarity {
        println!("Setting rarity filter: min={}, max={}", min, max);
        filter.min = min;
        filter.max = max;
    }

    if let Some(tags) = include {
        println!("Including tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    if let Some(tags) = exclude {
        println!("Excluding tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    filter
}

fn identify_file(path: &Path, regex: &Vec<DataEntry>, filter: &Filter) -> Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    identify_text(content, regex, filter);
    Ok(())
}

fn identify_directory(path: &Path, regex: &Vec<DataEntry>, filter: &Filter) -> Result<()> {
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

fn identify_text(text: String, regex_data: &Vec<DataEntry>, filter: &Filter) {
    let mut broken_regex_patterns = 0;
    for r in regex_data {
        if r.rarity < filter.min { 
            
        }
        
        let regex_pattern = &r.regex;
        // Find all matches
        // println!("Use regex pattern {}", regex_pattern);
        let re = match Regex::new(&regex_pattern) {
            Ok(r) => r,
            Err(_error) => {
                // TODO: Fix broken regex patterns and use other regex crate like fancy-regex
                broken_regex_patterns += 1;
                println!("Regex pattern for {} not valid.", r.name);
                // println!("Error: {}", _error);
                continue
            },
        };
        for mat in re.find_iter(&*text) {
            println!("Found match: {}", mat.as_str());
            println!("Type: {}", r.name.as_str());
        }
    }
    println!("Counted {}/{} broken regex patterns.", broken_regex_patterns, regex_data.len());
    // println!("Identifying text: {}", _text);
}

fn load_regex_pattern_data(file_path: &str) -> Result<Vec<DataEntry>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a Vec<DataEntry>
    let json_data: Vec<DataEntry> = serde_json::from_str(&contents)?;
    Ok(json_data)
}