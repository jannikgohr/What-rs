use crate::format::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub verbose: bool,
    pub only_text: bool,
    pub allow_duplicates: bool,
    pub pcapng: bool,
}