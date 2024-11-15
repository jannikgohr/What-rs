mod regex_pd;
mod filter;
mod identifier;
mod format;
mod sorter;
mod options;
mod cli;

use crate::cli::cli;
use crate::filter::Filter;
use crate::format::{get_format, output, OutputFormat};
use crate::identifier::{identify, Match};
use crate::regex_pd::TAGS;
use crate::sorter::Sorter;
use clap::Command;
use clap_complete::aot::{generate, Generator};
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use colored::Colorize;
use human_panic::setup_panic;
use std::{io, process};
use crate::options::Options;

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
        verbose: cli_matches.get_flag("verbose"),
        only_text: cli_matches.get_flag("only_text"),
        pcap: cli_matches.get_flag("pcap"),
    };

    options.format = get_format(&cli_matches.get_one::<String>("format"));

    if let Some(input) = cli_matches.get_one::<String>("input") {
        let mut matches: Vec<Match> = Vec::new();
        identify(input, &mut matches, &filter, &options).unwrap();
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
