extern crate getopts;

use getopts::Options;
use std::env;

// static SYNOPSIS: &str = "csplit [OPTION]... FILE PATTERN...";
// static NAME: &str = "csplit - split a file into sections determined by context lines";
static DESCRIPTION: &str = "Output pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ..., and output byte counts of each piece to standard output.";

fn print_usage(program: &str, opts: Options) {
    let brief = format!("{}\n{}", DESCRIPTION, program);
    print!("{}", opts.usage(&brief));
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
    opts.optflag("k", "keep-files", "use sprintf FORMAT instead of %02d");
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
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else {
        print_usage(&program, opts);
        return;
    }
}
