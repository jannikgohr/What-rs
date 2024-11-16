use crate::filter::Filter;
use crate::identifier::Match;
use crate::options::Options;
/*
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use std::error::Error;
use std::fs::File;
*/
use std::path::Path;

pub(crate) fn identify_pcap (
    _path: &Path,
    _matches: &mut Vec<Match>,
    _filter: &Filter,
    _options: &Options
) -> anyhow::Result<()> {
    unimplemented!();
    // Ok(())
}
