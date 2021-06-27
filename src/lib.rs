use std::{error::Error, fs};

const HELP: &str = r#"
USAGE
    minigrep <pattern> <file> [flags]

FLAGS:
    -i - Case insensitive search;
    -q - Quiet mode. Only outputs occurence count;
"#;

pub struct Program;

impl Program {
    pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
        let config = Config::new(&args)?;
        let file = FileReader::init(&config.filename)?;
        let searcher = if config.case_sensitive {
            file.search(config.query)
        } else {
            file.case_insensitive_search(config.query)
        };

        if !config.quiet {
            for line in &searcher {
                println!("{}", line);
            }
        }

        println!("{} occurences found.", searcher.len());

        Ok(())
    }
}

#[derive(Debug)]
struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    case_sensitive: bool,
    quiet: bool,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err(&HELP);
        }

        let mut case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        match args.split_at(3) {
            ([_p, query, filename], other_args) => {
                case_sensitive = if other_args.contains(&String::from("-i")) {
                    false
                } else {
                    case_sensitive
                };
                let quiet = other_args.contains(&"-q".to_string());

                Ok(Self {
                    query,
                    filename,
                    case_sensitive,
                    quiet,
                })
            }
            _ => Err(&HELP),
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

    fn case_insensitive_search(&self, pat: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        for line in self.contents.lines() {
            if line.to_lowercase().contains(&pat.to_lowercase()) {
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

    #[test]
    fn case_insensitive() {
        let query = "uSeR";
        let contents = "
            /usr/bin/aws
            /home/user/.cargo/bin/cargo
            /home/user/.cargo/bin/rustc
        ";

        assert_eq!(
            vec!["/home/user/.cargo/bin/cargo", "/home/user/.cargo/bin/rustc"],
            FileReader::case_insensitive_search(
                &FileReader {
                    contents: contents.to_string()
                },
                query
            )
        )
    }
}
