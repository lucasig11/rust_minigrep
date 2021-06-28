use std::{env, process};

use minigrep::Program;

fn main() {
    if let Err(e) = Program::run(env::args()) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
