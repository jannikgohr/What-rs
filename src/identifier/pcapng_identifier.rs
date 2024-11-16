use crate::filter::Filter;
use crate::identifier::{identify_text, Match};
use crate::options::Options;
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use std::fs::File;
use std::path::Path;

pub(crate) fn identify_pcapng (
    path: &Path,
    matches: &mut Vec<Match>,
    filter: &Filter,
    _options: &Options
) -> anyhow::Result<()> {
    println!("identify_pcapng");
    let file = File::open(path)?;
    let mut num_blocks = 0;
    let mut reader = PcapNGReader::new(65536, file).expect("PcapNGReader");
    loop {
        match reader.next() {
            Ok((offset, _block)) => {
                num_blocks += 1;
                let content = String::from_utf8_lossy(reader.data()).to_string();
                identify_text(content, matches, filter);
                reader.consume(offset);
            },
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete(_)) => {
                reader.refill().expect("incomplete PcapNGReader packet");
            },
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }
    Ok(())
}
