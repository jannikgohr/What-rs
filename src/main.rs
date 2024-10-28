mod regex_pd;
mod filter;
mod identifier;
mod options;

use crate::filter::{create_filter, parse_rarity, Filter};
use crate::identifier::identify;
use crate::options::Options;
use crate::regex_pd::load_regex_pattern_data;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use human_panic::setup_panic;
use std::process;

const HELP_TEMPLATE_FORMAT: &str = "\
{before-help}{name} {version}
{about-with-newline}
{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

fn main() {
    setup_panic!();
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
        print_tags().unwrap();
        process::exit(0);
    }

    let regex_data = load_regex_pattern_data("data/regex.json").unwrap();

    let rarity = matches
        .get_one::<String>("rarity")
        .map(|s| parse_rarity(s))
        .transpose()
        .context("Invalid rarity range format. Expected 'min:max'").unwrap();

    let filter: Filter = create_filter(
        rarity,
        !matches.get_flag("disable-borderless"),
        matches.get_one::<String>("include"),
        matches.get_one::<String>("exclude"),
    );

    let options: Options = Options {
        only_text: matches.get_flag("only_text"),
    };

    let input = matches.get_one::<String>("input").cloned();
    if input.is_none() {
        eprintln!("Text input expected. Run '--help' for usage.");
        process::exit(1);
    }

    // Determine if the input is text or a file/directory path
    if let Some(input) = matches.get_one::<String>("input") {
        identify(input, regex_data, filter, options).unwrap();
    } else {
        eprintln!("Input as text or file/directory path expected. Run '--help' for usage.");
        process::exit(1);
    }
}

fn print_tags() -> Result<()> {
    println!("Available Tags:");
    // TODO: Code to retrieve and print available tags goes here
    Ok(())
}




