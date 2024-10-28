use fancy_regex::Regex;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{fs, io, process};

#[derive(Debug, Deserialize)]
struct PatternData {
    name: String,
    regex: String,
    regex_no_anchor : Option<String>,
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
    borderless: bool
}

const HELP_TEMPLATE_FORMAT: &str = "\
{before-help}{name} {version}
{about-with-newline}
{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .help_template(HELP_TEMPLATE_FORMAT)
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
        .arg(
            Arg::new("disable-borderless")
                .short('d')
                .long("disable-borderless")
                .help("Disable borderless mode.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("tags") {
        print_tags()?;
        process::exit(0);
    }

    let regex_data = load_regex_pattern_data("data/regex.json")?;

    let rarity = matches
        .get_one::<String>("rarity")
        .map(|s| parse_rarity(s))
        .transpose()
        .context("Invalid rarity range format. Expected 'min:max'")?;

    let filter: Filter = create_filter(
        rarity,
        !matches.get_flag("disable-borderless"),
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
    borderless: bool,
    include: Option<&String>,
    exclude: Option<&String>,
) -> Filter {
    // TODO: Add include and exclude filter
    let mut filter: Filter = Filter { min: 0f64, max: 1f64, borderless };
    
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

fn identify_file(path: &Path, regex: &Vec<PatternData>, filter: &Filter) -> Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    identify_text(content, regex, filter);
    Ok(())
}

fn identify_directory(path: &Path, regex: &Vec<PatternData>, filter: &Filter) -> Result<()> {
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

fn identify_text(text: String, regex_data: &Vec<PatternData>, filter: &Filter) {
    for r in regex_data {
        if r.rarity < filter.min { 
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

fn load_regex_pattern_data(file_path: &str) -> Result<Vec<PatternData>, io::Error> {
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