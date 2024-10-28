use std::process;
use crate::identifier::Match;
use crate::options::{Options, OutputFormat};

pub fn output(matches: &Vec<Match>, options: &Options) {
    match options.format {
        OutputFormat::DEFAULT => { c_print_default(&matches) },
        OutputFormat::JSON => { c_print_json(&matches) },
        OutputFormat::PRETTY => { c_print_pretty(&matches) },
        OutputFormat::RAW => { c_print_raw(&matches) },
    }
}

pub fn get_format(format: &Option<&String>) -> OutputFormat {
    match format {
        Some(format) => {
            let f: &str = format.as_str();
            match f {
                "JSON" => { OutputFormat::JSON },
                "PRETTY" => { OutputFormat::PRETTY },
                "DEFAULT" => { OutputFormat::DEFAULT },
                "RAW" => { OutputFormat::RAW },
                &_ => {
                    eprintln!("Invalid format {}", f);
                    process::exit(1);
                }
            }
        }
        None => { OutputFormat::DEFAULT }
    }
}

fn c_print_default(matches: &Vec<Match>) {
    let mut output: Vec<&str> = Vec::new();
    for m in matches {
        output.push("\n");
        output.push("Matched on: ");
        output.push(&m.matched_on.as_str());
        output.push("\n");
        output.push("Name: ");
        output.push(&m.name.as_str());
        if let Some(description) = &m.description {
            output.push("\n");
            output.push("Description: ");
            output.push(&description.as_str());
        }
        if let Some(link) = &m.link {
            output.push("\n");
            output.push("Link: ");
            output.push(&link.as_str());
        }
        if let Some(exploit) = &m.exploit {
            output.push("\n");
            output.push("Exploit: ");
            output.push(&exploit.as_str());
        }
        output.push("\n");
    }
    let output_text = output.join("");
    println!("{}", output_text);
}

fn c_print_json(_matches: &Vec<Match>) {
    panic!("c_print_json: not implemented yet");
}

fn c_print_pretty(_matches: &Vec<Match>) {
    panic!("c_print_pretty: not implemented yet");
}

fn c_print_raw(_matches: &Vec<Match>) {
    panic!("c_print_raw: not implemented yet");
}