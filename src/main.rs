use bio::io::fasta;
use clap::{arg_enum};
use structopt;
use structopt::StructOpt;
use std::io;
use regex::Regex;
// use std::str::FromStr;
use std::str;

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Alphabet {
        Protein,
        NucleicAcid
    }
}

// struct MultiPattern {
//     pattern: Regex,
//     count: u32
// }
//
// impl FromStr for MultiPattern {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//
//     }
// }


// impl From<Alphabet> for Regex {
//     fn from(alpha: Alphabet) -> Self {
//         match alpha {
//             Alphabet::NucleicAcid => Regex(),
//             Alphabet::Protein => Regex()
//         }
//     }
// }
#[derive(Debug, StructOpt)]
#[structopt(version = "0.1.0", author = "Michael R Milton <michael.r.milton@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    // #[structopt(short, long, default_value = "protein", possible_values = & Alphabet::variants(), case_insensitive = true)]
    // alphabet: Alphabet,
    // #[structopt()]
    patterns: Vec<Regex>
}

fn main() {
    let opt = Opts::from_args();
    let reader = fasta::Reader::new(io::stdin());
    let mut writer = fasta::Writer::new(io::stdout());
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        if opt.patterns.iter().all(|pat| pat.is_match(str::from_utf8(record.seq()).unwrap())){
            &writer.write_record(&record);
        }
    }
    // println!("{:?}", opt.alphabet);
}
