use std::io;
use std::slice::Split;
use std::str;
use std::str::FromStr;

use bio::io::fasta;
use regex::{Regex};
use structopt;
use structopt::StructOpt;

use itertools::Itertools;

use anyhow::{Result, Error, anyhow};

struct NamedQuery {
    name: String,
    patterns: Vec<Regex>
}

impl NamedQuery {
    fn vec_from(vec: Vec<NamedRegex>) -> Vec<Self>{
        vec.into_iter().sorted_by_key(|it| it.name.clone()).group_by(|it| it.name.clone()).into_iter().map(|(key, items)| {
            NamedQuery{
                name: key.clone(),
                patterns: items.into_iter().map(|nr| nr.pattern).collect()
            }
        }).collect()
    }
}

#[derive(Debug)]
struct NamedRegex {
    name: String,
    pattern: Regex,
}

impl FromStr for NamedRegex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split('=').collect::<Vec<&str>>().as_slice(){
            [key, value] => {
                Ok(NamedRegex {
                    name: key.clone().into(),
                    pattern: Regex::from_str(value.clone())?
                })
            }
            _ => Err(anyhow!("Failed to parse regex option."))
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    version = "0.1.0",
    author = "Michael R Milton <michael.r.milton@gmail.com>"
)]
/// FastaMultiGrep
///
/// Choose a subcommand.
enum Opts {
    /// Searches each sequence in a fasta file for a combination of regular expressions, and outputs
    /// any sequences that match the pattern in fasta format.
    ///
    /// The fasta file is read in stdin, so you will have to pipe it into this tool.
    /// Output is produced in stdout in fasta format
    /// Therefore you will likely want to pipe the output to a file
    Filter {
        /// A list of regular expressions, ideally enclosed in quotation marks. The supported syntax is documented here: https://docs.rs/regex/latest/regex/#syntax
        patterns: Vec<Regex>,
    },
    /// Searches each sequence in a fasta file for one or more groups of regular expressions, and
    /// outputs a table of hits in CSV format
    ///
    /// The fasta file is read in stdin, so you will have to pipe it into this tool.
    /// Output is produced as a CSV file to stdout, with headers
    /// Therefore you will likely want to pipe the output to a file
    Hits {
        /// A list of named regular expressions in the form "seqname=(some|regex)", ideally enclosed in quotation marks.
        /// You can re-use a pattern name to request the intersection of the two patterns, ie `foo=ABC` `foo=DEF` will only match
        /// a sequence for `foo` if it contains both `ABC` and `DEF`
        /// The regex syntax is documented here: https://docs.rs/regex/latest/regex/#syntax
        patterns: Vec<NamedRegex>
    }
}

fn main() {
    let reader = fasta::Reader::new(io::stdin());
    let opt = Opts::from_args();
    match opt {
        Opts::Filter{patterns} => {
            let mut writer = fasta::Writer::new(io::stdout());
            for result in reader.records() {
                let record = result.expect("Error during fasta record parsing");
                if patterns
                    .iter()
                    .all(|pat| pat.is_match(str::from_utf8(record.seq()).unwrap()))
                {
                    &writer.write_record(&record);
                }
            }
        }
        Opts::Hits{patterns} => {
            let patterns = NamedQuery::vec_from(patterns);
            let mut wtr = csv::WriterBuilder::new().from_writer(io::stdout());
            wtr.write_record(&["seq", "pattern", "match"]).unwrap();

            for result in reader.records() {
                let record = result.expect("Error during fasta record parsing");
                for pattern in &patterns {
                    let mtch =  (&pattern.patterns)
                        .into_iter()
                        .all(|pat| pat.is_match(str::from_utf8(record.seq()).unwrap()));
                    wtr.write_record(&[record.id(), &pattern.name, &mtch.to_string()]).unwrap();
                }
            }
        }
    }
}
