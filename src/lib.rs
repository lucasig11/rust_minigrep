use std::{error::Error, fs};

pub struct Program;

impl Program {
    pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
        let config = Config::new(&args)?;
        let file = FileReader::init(&config.filename)?;
        for line in file.search(config.query) {
            println!("{}", line);
        }
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

struct FileReader {
    contents: String,
}

impl FileReader {
    fn init(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(&filename)?;
        Ok(Self {
            contents: contents.trim().to_string(),
        })
    }

    fn search(&self, pat: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        for line in self.contents.lines() {
            if line.contains(pat) {
                result.push(line.trim());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "user";
        let contents = "
            /usr/bin/aws
            /home/user/.cargo/bin/cargo
            /home/user/.cargo/bin/rustc
        ";

        assert_eq!(
            vec!["/home/user/.cargo/bin/cargo", "/home/user/.cargo/bin/rustc"],
            FileReader::search(
                &FileReader {
                    contents: contents.to_string()
                },
                query
            )
        )
    }
}
