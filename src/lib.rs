use std::{env, error::Error, fs};

const HELP: &str = r#"
USAGE
    minigrep <pattern> <file> [flags]

FLAGS:
    -i - Case insensitive search;
    -q - Quiet mode. Only outputs occurence count;
"#;

pub struct Program;

impl Program {
    pub fn run(args: env::Args) -> Result<(), Box<dyn Error>> {
        let config = Config::new(args)?;
        let content = fs::read_to_string(&config.filename)?;
        let file = FileReader::init(&content)?;
        let searcher = if config.case_sensitive {
            file.search(&config.query)
        } else {
            file.case_insensitive_search(&config.query)
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
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
    quiet: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(&HELP),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(&HELP),
        };

        let other_args: Vec<_> = args.collect();

        let case_sensitive =
            std::env::var("CASE_INSENSITIVE").is_err() && !other_args.contains(&String::from("-i"));

        let quiet = other_args.contains(&"-q".to_string());

        Ok(Self {
            query,
            filename,
            case_sensitive,
            quiet,
        })
    }
}

#[derive(Clone, Copy)]
struct FileReader<'a> {
    contents: &'a str,
}

impl<'a> FileReader<'a> {
    fn init(contents: &'a str) -> Result<Self, Box<dyn Error>> {
        Ok(Self { contents })
    }

    fn search(&self, pat: &str) -> Vec<&str> {
        self.contents
            .lines()
            .filter(|&line| line.contains(&pat))
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
    }

    fn case_insensitive_search(&self, pat: &str) -> Vec<&str> {
        self.contents
            .lines()
            .filter(|&line| line.to_lowercase().contains(&pat.to_lowercase()))
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
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
            FileReader::search(&FileReader { contents }, query)
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
            FileReader::case_insensitive_search(&FileReader { contents }, query)
        )
    }
}
