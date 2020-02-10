extern crate getopts;
use getopts::Options;
use std::env;

fn usage(prog: &str, opts: Options) -> String {
    let brief = format!("USAGE: {}", prog);
    return format!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("b", "build", "Build an index for source text");
    opts.optflag("h", "help",  "Print the help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => {
            eprint!("{}", usage(&args[0].clone(), opts));
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print!("{}", usage(&args[0].clone(), opts));
        std::process::exit(0);
    } else if matches.opt_present("b") {
        println!("Building index currently not supported");
        std::process::exit(1);
    } else {
        print!("{}", usage(&args[0].clone(), opts));
        std::process::exit(0);
    }
}
