use std::process;
use fancy_regex::Regex;
use colored::*;
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
    let mut output: Vec<String> = Vec::new();
    for m in matches {
        output.push(String::from("\n"));
        output.push(String::from("Matched on: "));
        output.push(m.matched_on.clone());
        output.push(String::from("\n"));
        output.push(String::from("Name: "));
        output.push(m.name.clone());
        if let Some(description) = &m.description {
            output.push(String::from("\n"));
            output.push(String::from("Description: "));
            output.push(description.clone());
        }
        if let Some(link) = &m.link {
            output.push(String::from("\n"));
            output.push(String::from("Link: "));
            output.push(link.clone());
            output.push(m.matched_on.as_str().replace(" ", ""));
        }
        if let Some(exploit) = &m.exploit {
            output.push(String::from("\n"));
            output.push(String::from("Exploit: "));
            output.push(exploit.clone());
        }
        output.push(String::from("\n"));
    }
    let output_text = output.join("");
    display_wikitext(output_text.as_str());
    //println!("{}", output_text);
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

fn display_wikitext(wikitext: &str) {
    let color_re = Regex::new(r"\[#([A-Fa-f0-9]{6})\](.*?)\[/#\1\]").unwrap();
    let link_re = Regex::new(r"\[link=(.*?)\](.*?)\[/link\]").unwrap();

    let mut result = String::from(wikitext);

    // Process color tags
    for cap in color_re.captures_iter(wikitext) {
        match cap {
            Ok(captures) => {
                let color_code = &captures[1];
                let text = &captures[2];
                let colored_text = text.truecolor(
                    u8::from_str_radix(&color_code[0..2], 16).unwrap(),
                    u8::from_str_radix(&color_code[2..4], 16).unwrap(),
                    u8::from_str_radix(&color_code[4..6], 16).unwrap(),
                );
                result = result.replace(&captures[0], &colored_text.to_string());
            }
            Err(e) => {
                eprintln!("Regex error: {}", e); // Log the error if needed
            }
        }
    }

    // Process link tags
    for cap in link_re.captures_iter(wikitext) {
        match cap {
            Ok(captures) => {
                let url = &captures[1];
                let link_text = &captures[2];
                let formatted_link = format!("{} ({})", link_text.underline(), url);
                result = result.replace(&captures[0], &formatted_link);
            }
            Err(e) => {
                eprintln!("Regex error: {}", e); // Log the error if needed
            }
        }
    }

    println!("{}", result);
}