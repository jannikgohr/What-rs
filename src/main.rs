use std::path::Path;
use anyhow::{Context, Result};
use clap::{Arg, Command};
use std::{fs, process};

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

    let rarity = matches
        .get_one::<String>("rarity")
        .map(|s| parse_rarity(s))
        .transpose()
        .context("Invalid rarity range format. Expected 'min:max'")?;

    let filter = create_filter(
        rarity,
        matches.get_one::<String>("include"),
        matches.get_one::<String>("exclude"),
    )?;

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
                identify_file(path, filter)?;
            } else if path.is_dir() {
                identify_directory(path, filter)?;
            } else {
                eprintln!("Input path is not a file or directory");
                process::exit(1);
            }
        } else {
            // Handle as plain text
            identify_text(input.to_string(), filter);
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

fn parse_rarity(rarity: &str) -> Result<(f32, f32)> {
    let parts: Vec<&str> = rarity.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid rarity format. \
        Format must be 'min:max', where min and max are decimal numbers seperated by a colon.");
    }
    let min = parts[0].parse::<f32>()?;
    let max = parts[1].parse::<f32>()?;
    if min < 0f32 || max > 1f32 {
        anyhow::bail!("Invalid rarity range. Range must be between 0 and 1 inclusive.");
    }
    Ok((min, max))
}

fn create_filter(
    rarity: Option<(f32, f32)>,
    include: Option<&String>,
    exclude: Option<&String>,
) -> Result<()> {
    // Logic to create a filter based on rarity, include, and exclude values
    if let Some((min, max)) = rarity {
        println!("Setting rarity filter: min={}, max={}", min, max);
    }

    if let Some(tags) = include {
        println!("Including tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    if let Some(tags) = exclude {
        println!("Excluding tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    // Placeholder filter creation
    Ok(())
}

fn identify_file(path: &Path, _filter: ()) -> Result<()> {
    // TODO: Better error handling
    println!("Identifying file {:?}", path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    identify_text(content, _filter);
    Ok(())
}

fn identify_directory(path: &Path, _filter: ()) -> Result<()> {
    println!("Identifying directory: {:?}", path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            identify_file(&file_path, _filter)?;
        } else if file_path.is_dir() {
            identify_directory(&file_path, _filter)?;
        }
    }
    Ok(())
}

fn identify_text(text: String, _filter: ()) {
    // Placeholder for text identification logic
    // println!("Identifying text: {}", text);
}
