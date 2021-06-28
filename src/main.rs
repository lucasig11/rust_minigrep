use std::{env, process};

use minigrep::Program;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = Program::run(&args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
