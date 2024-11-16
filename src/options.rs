use crate::format::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub verbose: bool,
    pub only_text: bool,
    pub pcap: bool,
    pub pcapng: bool,
}