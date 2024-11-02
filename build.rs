use fancy_regex::Regex as Fancy;
use regex::Regex;
use serde::Deserialize;
use std::fmt::Write;
use std::{env, fs, path::Path};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PatternData {
    name: String,
    regex: String,
    #[serde(skip_deserializing)]
    regex_no_anchor: String,
    plural_name: bool,
    description: Option<&'static str>,
    exploit: Option<String>,
    rarity: f32,
    url: Option<&'static str>,
    tags: Vec<&'static str>,
}

fn main() {
    let mut data: Vec<PatternData> = serde_json::from_str(include_str!("./data/regex.json")).unwrap();

    data.iter_mut().for_each(|d| {
        d.regex_no_anchor = Fancy::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for regex_no_anchor")
            .replace(&d.regex, "")
            .to_string();
        d.regex_no_anchor = Fancy::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for regex_no_anchor")
            .replace(&d.regex_no_anchor, "")
            .to_string();
    });

    data.retain(|r| Regex::new(&r.regex).is_ok() && Regex::new(&r.regex_no_anchor).is_ok());

    let mut data_str = format!("{:?}", data);
    // we want reference to [], i.e. &[]
    data_str = data_str.replace("tags: [", "tags: &[");

    let regex_str: String = data.iter().fold(String::new(), |mut output, d| {
        let _ = write!(
            output,
            "\tLazy::new(|| Regex::new({:?}).unwrap()),\n",
            d.regex
        );
        output
    });

    let regex_no_anchor_str: String = data.iter().fold(String::new(), |mut output, d| {
        let _ = write!(
            output,
            "\tLazy::new(|| Regex::new({:?}).unwrap()),\n",
            d.regex_no_anchor
        );
        output
    });

    let count = data.len();
    let final_str = format!(
        "pub const DATA: [PatternData; {count}] = {data_str};"
    );
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    fs::write(dest_path, final_str).unwrap();

    let mut final_str = format!(
        "pub static REGEX: [Lazy<Regex>; {count}] = [\n{regex_str}];\n"
    );
    final_str += "\n";
    final_str += format!(
        "pub static REGEX_NO_ANCHOR: [Lazy<Regex>; {count}] = [\n{regex_no_anchor_str}];"
    ).as_str();
    let regex_dest_path = Path::new(&out_dir).join("regex_data.rs");
    fs::write(regex_dest_path, final_str).unwrap();
}
