use std::{error::Error, fs};

pub struct Program {}

impl Program {
    pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
        let config = Config::new(&args)?;
        let file = FileReader::init(&config.filename)?;
        println!("{} {}", &file.filename, &file.contents);
        Ok(())
    }
}

#[derive(Debug)]
struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &str> {
        match args.get(1..) {
            Some([query, filename, ..]) => Ok(Self { query, filename }),
            _ => Err("Missing arguments. Usage: minigrep <pattern> <file>"),
        }
    }
}

struct FileReader<'a> {
    contents: String,
    filename: &'a str,
}

impl<'a> FileReader<'a> {
    fn init<'l>(filename: &'a str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(&filename)?;
        Ok(Self { contents, filename })
    }
}
