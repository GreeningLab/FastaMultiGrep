use std::io;
use std::str;

use bio::io::fasta;
use clap::arg_enum;
use regex::Regex;
use structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    version = "0.1.0",
    author = "Michael R Milton <michael.r.milton@gmail.com>"
)]
/// Searches each sequence in a fasta file for a combination of regular expressions.
///
/// The fasta file is read in stdin, so you will have to pipe it into this tool.
/// Output is produced in stdout as a fasta file which only contains matches.
/// Therefore you will likely want to pipe the output to a file
/// Note that tool assumes you want the intersection of pattern hits, ie hits that include pattern A AND pattern B AND pattern C, in any order.
/// If you want the union of hits, or you want a specific order, use regex features to achieve this
struct Opts {
    /// A list of regular expressions, ideally enclosed in quotation marks. The supported syntax is documented here: https://docs.rs/regex/latest/regex/#syntax
    patterns: Vec<Regex>,
}

fn main() {
    let opt = Opts::from_args();
    let reader = fasta::Reader::new(io::stdin());
    let mut writer = fasta::Writer::new(io::stdout());
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        if opt
            .patterns
            .iter()
            .all(|pat| pat.is_match(str::from_utf8(record.seq()).unwrap()))
        {
            &writer.write_record(&record);
        }
    }
}
