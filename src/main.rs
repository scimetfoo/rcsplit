extern crate getopts;
extern crate regex;

use getopts::Matches;
use getopts::Options;
use regex::Regex;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

// static SYNOPSIS: &str = "csplit [OPTION]... FILE PATTERN...";
// static NAME: &str = "csplit - split a file into sections determined by context lines";
static DESCRIPTION: &str = "Output pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ..., and output byte counts of each piece to standard output.";

pub struct Behaviour {
    suffix_format: String,
    prefix: String,
    digits: u64,
    filename: String,
    splitter: String,
    repetition_count: u64,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("{}\n{}", DESCRIPTION, program);
    print!("{}", opts.usage(&brief));
}

fn parse_args(opts: &Options, args: Vec<String>) -> Matches {
    return match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "display help and exit");
    opts.optopt(
        "b",
        "suffix-format",
        "use sprintf FORMAT instead of %02d",
        "FORMAT",
    );
    opts.optopt("f", "prefix", "use PREFIX instead of 'xx'", "PREFIX");
    opts.optflag("k", "keep-files", "do not remove output files on errors ");
    opts.optflag(
        "",
        "suppress-matched",
        "suppress the lines matching PATTERN",
    );
    opts.optopt(
        "n",
        "digits",
        "use specified number of digits instead of 2",
        "DIGITS",
    );
    opts.optflag("s", "quiet", "do not print counts of output file sizes");
    opts.optflag("z", "elide-empty-files", "remove empty output files");
    opts.optflag("", "version", "output version information and exit");

    let matches = parse_args(&opts, args);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut behaviour = Behaviour {
        suffix_format: "".to_owned(),
        prefix: "".to_owned(),
        digits: 2,
        filename: "".to_owned(),
        splitter: "".to_owned(),
        repetition_count: 1,
    };

    behaviour.suffix_format = matches.opt_get_default("b", "".to_owned()).unwrap();
    behaviour.prefix = matches.opt_get_default("f", "".to_owned()).unwrap();
    behaviour.digits = matches.opt_get_default("n", 2).unwrap();

    let mut opts_iter = matches.free.iter();

    behaviour.filename = match opts_iter.next() {
        None => {
            println!("No file name provided");
            std::process::exit(1);
        }
        Some(a) => a.to_owned(),
    };

    behaviour.splitter = match opts_iter.next() {
        None => "".to_owned(),
        Some(a) => a.to_owned(),
    };

    let re = Regex::new(r#"[\{]+([^}]+)[\}]"#).unwrap(); // becase no look-aheads
    behaviour.repetition_count = match opts_iter.next() {
        None => 1,
        Some(a) => {
            let caps = re.captures(a.trim()).unwrap();
            caps.get(1)
                .map_or(1, |m| m.as_str().parse().expect("Bad repetition count"))
        }
    };

    rcsplit(&behaviour);
}

}
