mod regex_pd;
mod filter;
mod identifier;
mod format;
mod sorter;

use crate::filter::Filter;
use crate::format::{get_format, output, Options, OutputFormat};
use crate::identifier::{identify, Match};
use crate::sorter::Sorter;
use crate::regex_pd::TAGS;
use clap::{Arg, Command};
use clap_complete::aot::{generate, Generator};
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use human_panic::setup_panic;
use std::{io, process};
use colored::Colorize;

const HELP_TEMPLATE_FORMAT: &str = "\
{before-help}{name} {version}

{about-with-newline}
{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
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
                .help("Only show matches with these tags.")
                .default_value(""),
        )
        .arg(
            Arg::new("exclude")
                .short('e')
                .long("exclude")
                .help("Exclude matches with these tags.")
                .default_value(""),
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
        .arg(
            Arg::new("format")
                .long("format")
                .help("Output format.")
                .value_parser(["default", "json", "pretty"]),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .help("Reverse the sorting order.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_parser(["name", "rarity", "matched", "none"])
                .default_value("none")
                .default_missing_value("none")
                .help("Filter by key name."),
        )
        .arg(
            Arg::new("generate")
                .long("generate")
                .help("Generate a shell completion file for the specified shell")
                .value_parser(["bash", "zsh", "fish", "powershell", "elvish"]),
        )
}

fn main() {
    setup_panic!();

    clap_complete::CompleteEnv::with_factory(cli)
        .completer("exhaustive")
        .complete();

    let cli_matches = cli().get_matches();
    if let Some(generator) = cli_matches.get_one::<String>("generate") {
        let mut cmd = cli();
        eprintln!("Generating completion file for {generator}...");

        match generator.as_str() {
            "bash" => print_completions(Bash, &mut cmd),
            "zsh" => print_completions(Zsh, &mut cmd),
            "fish" => print_completions(Fish, &mut cmd),
            "powershell" => print_completions(PowerShell, &mut cmd),
            "elvish" => print_completions(Elvish, &mut cmd),
            _ => eprintln!("Unknown shell specified."),
        }
        return;
    }

    if cli_matches.get_flag("tags") {
        print_tags();
        process::exit(0);
    }

    let input = cli_matches.get_one::<String>("input").cloned();
    if input.is_none() {
        if cli_matches.args_present() {
            cli().help_template("{usage-heading} {usage}\n\n{all-args}{after-help}")
                .print_help().unwrap();
        } else {
            println!("{} (Version: {})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            println!("\n{}", env!("CARGO_PKG_DESCRIPTION"));
            println!("Made by {}", env!("CARGO_PKG_AUTHORS"));
            println!("For more information see {}", env!("CARGO_PKG_HOMEPAGE"));
            eprintln!("\nRun '--help' for usage.");
        }
        process::exit(1);
    }

    let filter = Filter::default()
        .rarity(cli_matches.get_one::<String>("rarity").unwrap())
        .borderless(!cli_matches.get_flag("disable-borderless"))
        .include(cli_matches.get_one::<String>("include").unwrap_or(&String::from("")))
        .exclude(cli_matches.get_one::<String>("exclude").unwrap_or(&String::from("")));

    let mut options: Options = Options {
        format: OutputFormat::DEFAULT,
    };

    options.format = get_format(&cli_matches.get_one::<String>("format"));

    if let Some(input) = cli_matches.get_one::<String>("input") {
        let mut matches: Vec<Match> = Vec::new();
        identify(input, &mut matches, &filter, cli_matches.get_flag("only_text")).unwrap();
        Sorter::default()
            .key(cli_matches.get_one::<String>("key").unwrap())
            .reverse(cli_matches.get_flag("reverse"))
            .sort(&mut matches);
        output(&matches, &options)
    } else {
        eprintln!("Input as text or file/directory path expected. Run '--help' for usage.");
        process::exit(1);
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn print_tags() {
    println!("{}\n", "Available Tags:".purple());
    println!("{}", TAGS
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>()
        .join("\n")
    );
}
