use bio::io::fasta;
use clap::{arg_enum};
use structopt;
use structopt::StructOpt;
use std::io;
use regex::Regex;

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Alphabet {
        Protein,
        NucleicAcid
    }
}

#[derive(Debug, StructOpt)]
#[structopt(version = "0.1.0", author = "Michael R Milton <michael.r.milton@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[structopt(short, long, default_value = "protein", possible_values = & Alphabet::variants(), case_insensitive = true)]
    alphabet: Alphabet,
}

impl From<Alphabet> for Regex {
    fn from(alpha: Alphabet) -> Self {
        match alpha {
            Alphabet::NucleicAcid => Regex(),
            Alphabet::Protein => Regex()
        }
    }
}

fn main() {
    let opt = Opts::from_args();
    let mut reader = fasta::Reader::new(io::stdin());
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
    }
    println!("{:?}", opt.alphabet);
}
