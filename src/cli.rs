use clap::{Arg, Command};

const HELP_TEMPLATE_FORMAT: &str = "\
{before-help}{name} {version}

{about-with-newline}
{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

pub fn cli() -> Command {
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
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show more information.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pcap")
                .long("pcap")
                .help("Analyze a pcap file.")
                .action(clap::ArgAction::SetTrue),
        )
}