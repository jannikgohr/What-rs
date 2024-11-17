use crate::filter::Filter;
use crate::identifier::{Identifier, Match};
use crate::options::Options;
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::{PcapError, PcapNGReader};
use std::fs::File;
use std::path::Path;


pub(crate) fn identify_pcapng (
    path: &Path,
    matches: &mut Vec<Match>,
    filter: &Filter,
    options: &Options
) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let mut identifier = Identifier::new();
    let mut reader = PcapNGReader::new(65536, file).expect("PcapNGReader");

    loop {
        match reader.next() {
            Ok((offset, _block)) => {
                let content = String::from_utf8_lossy(reader.data()).to_string();
                // println!("{}", content);
                identifier.identify_text(content, matches, filter, options);
                reader.consume(offset);
            },
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete(_)) => {
                reader.refill().expect("incomplete PcapNGReader packet");
            },
            Err(PcapError::BufferTooSmall) => {
                reader.refill().expect("Buffer to small and failed recovery");
            }
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }

    Ok(())
}
