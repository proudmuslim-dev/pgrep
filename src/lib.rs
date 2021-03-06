pub mod util;

use std::{env, error::Error, fmt, fmt::Display, fs, fs::File, io::ErrorKind, process};


use linked_hash_map::LinkedHashMap;
use regex::Regex;
use util::stderr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let conf = Config {
            query: "a".to_owned(),
            filename: "a.txt".to_owned(),
            case_sensitive: false,
        };

        let content = get_content(&conf).expect("Failed to get content!");
        let mut x: LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(1, "a".to_owned());

        assert_eq!(x, search(conf, content))
    }

    #[test]
    fn one_case_insensitive_result() {
        let conf = Config {
            query: "A".to_owned(),
            filename: "a.txt".to_owned(),
            case_sensitive: true,
        };

        let content = get_content(&conf).expect("Failed to get content!");
        let mut x: LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(1, "a".to_owned());

        assert_eq!(x, search(conf, content))
    }

    #[test]
    fn multiple_results() {
        let conf = Config {
            query: "a".to_owned(),
            filename: "file.txt".to_owned(),
            case_sensitive: false,
        };

        let content = get_content(&conf).expect("Failed to get content!");
        let mut x: LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(1, "Random stuff".to_owned());
        x.insert(3, "aaaaa".to_owned());
        x.insert(5, "ae".to_owned());
        x.insert(
            9,
            "I categorically deny having triskaidekaphobia.".to_owned(),
        );

        assert_eq!(x, search(conf, content))
    }

    #[test]
    fn regex() {
        let conf = Config {
            query: "\\b\\w{13}\\b".to_owned(),
            filename: "file.txt".to_owned(),
            case_sensitive: false,
        };

        let content = get_content(&conf).expect("Failed to get content");

        let mut x: LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(
            9,
            "I categorically deny having triskaidekaphobia.".to_owned(),
        );

        assert_eq!(x, search(conf, content))
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query supplied!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file supplied!"),
        };

        let case_sensitive = !env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Query: '{}' \nFile: {}",
            self.query.as_str(),
            self.filename.as_str()
        )
    }
}

// TODO: Move the rest of the functions into the impl block of Config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("{}", config.case_sensitive);
    let content = crate::get_content(&config).unwrap();
    let matches = crate::search(config, content);
    for (line, value) in matches {
        println!("{}: {}", line, value)
    }

    Ok(())
}

fn search(config: Config, content: String) -> LinkedHashMap<usize, String> {
    let mut results = LinkedHashMap::new();
    let mut line_num: usize = 1;

    if !config.case_sensitive {
        let re = Regex::new(config.query.as_str()).unwrap();
        for line in content.lines() {
            if re.is_match(line) {
                results.insert(line_num, line.to_string());
            }
            line_num += 1;
        }
    } else {
        let re = Regex::new(config.query.to_ascii_lowercase().as_str()).unwrap();

        for line in content.to_ascii_lowercase().lines() {
            if re.is_match(line) {
                results.insert(line_num, line.to_string());
            }
            line_num += 1;
        }
    }

    if results.len() != 0 {
        results
    } else {
        stderr(format!("Pattern '{}' is not present in file.", config.query).as_str())
            .expect("Failed to print error message for search!");
        process::exit(1)
    }
}

fn get_content(config: &Config) -> Option<String> {
    let _f = File::open(&config.filename).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            stderr(format!("File '{}' not found, exiting...", &config.filename).as_str())
                .expect("Failed to print error message for get_content!");
            process::exit(1);
        }
        ErrorKind::PermissionDenied => {
            stderr(format!("Missing permissions to open file '{}'.", &config.filename).as_str())
                .expect("Failed to print error message for get_content!");
            process::exit(1);
        }
        _ => panic!("Problem opening the file: {:?}", error),
    });

    let content = fs::read_to_string(&config.filename).expect("Unable to read file");
    Some(content)
}
