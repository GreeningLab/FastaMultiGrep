# FastaMultiGrep

Find multiple motifs in a fasta file

## Installation

* Download an applicable exe from the [releases page](https://github.com/GreeningLab/FastaMultiGrep/release)
* Or, if you want the latest version, or you have an architecture that isn't included in the releases, you can run this
  tool locally:
    * Make sure `rustc` is installed
    * `git clone https://github.com/GreeningLab/FastaMultiGrep.gitFastaMultiGrep`
    * `cd FastaMultiGrep`
    * `cargo run -- <arguments here>` (yes, you need to include the double dash literally `--`)

## Usage

```
fasta_multi_grep 0.1.0
Michael R Milton <michael.r.milton@gmail.com>
Searches each sequence in a fasta file for a combination of regular expressions.

The fasta file is read in stdin, so you will have to pipe it into this tool. Output is produced in stdout as a fasta
file which only contains matches. Therefore you will likely want to pipe the output to a file Note that tool assumes you
want the intersection of pattern hits, ie hits that include pattern A AND pattern B AND pattern C, in any order. If you
want the union of hits, or you want a specific order, use regex features to achieve this

USAGE:
    fasta_multi_grep [patterns]...

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


ARGS:
    <patterns>...    
            A list of regular expressions, ideally enclosed in quotation marks. The supported syntax is documented here:
            https://docs.rs/regex/latest/regex/#syntax

```