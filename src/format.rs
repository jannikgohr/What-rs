use crate::identifier::Match;
use crate::options::{Options, OutputFormat};
use colored::*;
use fancy_regex::Regex;
use std::process;
use tabled::{settings::{object::Rows, style::Style, themes::Colorization, Color}, Table, Tabled};

#[derive(Tabled)]
struct MatchTableRow {
    #[tabled(rename = "Matched Text")]
    matched_on: String,
    #[tabled(rename = "Identified as")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
}

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
            let f: String = format.to_uppercase();
            match f.as_str() {
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
}

fn c_print_json(matches: &Vec<Match>) {
    let json_output = serde_json::to_string(matches).unwrap();
    println!("{}", json_output);
}

fn c_print_pretty(matches: &Vec<Match>) {
    let rows = matches_to_table_rows(matches);
    let mut table = Table::new(rows);
    table
        .with(Style::modern())
        .with(Colorization::exact([Color::FG_MAGENTA], Rows::first()));
    display_wikitext(table.to_string().as_str());
    //println!("{}", table);
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
                let formatted_link = format!("{} ({}){}", link_text.underline(), url,
                                             " ".repeat(30));
                result = result.replace(&captures[0], &formatted_link.to_string());
            }
            Err(e) => {
                eprintln!("Regex error: {}", e); // Log the error if needed
            }
        }
    }

    println!("{}", result);
}

fn matches_to_table_rows(matches: &Vec<Match>) -> Vec<MatchTableRow> {
    let mut result = Vec::new();

    for m in matches {

        let matched_on = m.matched_on.clone();
        let name = m.name.clone();
        let description;
        if let Some(m_desc) = &m.description {
            description = m_desc.to_string()
        } else if let Some(link) = &m.link {
            let mut desc = String::from("Click here to analyse in the browser\n");
            desc += link.clone().as_str();
            desc += &*m.matched_on.as_str().replace(" ", "");
            description = desc
        } else {
            description = String::from("None");
        }
        let row: MatchTableRow = MatchTableRow {
            matched_on,
            name,
            description,
        };
        result.push(row);
    }
    result
}