mod regex_pd;
mod filter;
mod identifier;

use crate::identifier::{identify_directory, identify_file, identify_text};
use crate::regex_pd::load_regex_pattern_data;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde::Deserialize;
use std::io::Read;
use std::path::Path;
use std::process;
use crate::filter::{create_filter, Filter};

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
            Arg::new("input")
                .help("Input to identify. Input can be text, a file or directory.")
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

    let input = matches.get_one::<String>("input").cloned();
    if input.is_none() {
        eprintln!("Text input expected. Run '--help' for usage.");
        process::exit(1);
    }

    // Determine if the input is text or a file/directory path
    if let Some(input) = matches.get_one::<String>("input") {
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
        eprintln!("Input as text or file/directory path expected. Run '--help' for usage.");
        process::exit(1);
    }

    Ok(())
}

fn print_tags() -> Result<()> {
    println!("Available Tags:");
    // TODO: Code to retrieve and print available tags goes here
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



