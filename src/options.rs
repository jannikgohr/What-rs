pub enum OutputFormat {
    // TODO: implement all
    DEFAULT,
    JSON,
    PRETTY,
    RAW
}

pub struct Options {
    pub only_text: bool,
    pub format: OutputFormat,
}