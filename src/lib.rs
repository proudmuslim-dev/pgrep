use linked_hash_map::LinkedHashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::ErrorKind;
use std::{fmt, fs, process};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let conf = Config {
            query: "a".to_owned(),
            filename: "a.txt".to_owned(),
        };

        let content = get_content(&conf).unwrap();
        let mut x : LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(1, "a".to_owned());

        assert_eq!(x, search(conf.query.as_str(), content.as_str()))
    }

    #[test]
    fn multiple_results() {
        let conf = Config {
            query: "a".to_owned(),
            filename: "file.txt".to_owned(),
        };

        let content = get_content(&conf).unwrap();
        let mut x : LinkedHashMap<usize, String> = LinkedHashMap::new();
        x.insert(1, "Random stuff".to_owned());
        x.insert(3, "aaaaa".to_owned());
        x.insert(5, "ae".to_owned());

        assert_eq!(x, search(conf.query.as_str(), content.as_str()))
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
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

fn get_content(config: &Config) -> Option<String> {
    let _f = File::open(&config.filename).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            print_stderr(format!("File '{}' not found, exiting...", &config.filename)).unwrap();
            process::exit(1);
        }
        ErrorKind::PermissionDenied => {
            print_stderr(format!("Missing permissions to open file '{}'.", &config.filename)).unwrap();
            process::exit(1);
        }
        _ => panic!("Problem opening the file: {:?}", error),
    });

    let content = fs::read_to_string(&config.filename).expect("Unable to read file");
    Some(content)
}

fn print_stderr(message: String) -> Result<(), Box<dyn Error>> {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    eprintln!("{}", message);

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = crate::get_content(&config).unwrap();
    let matches = crate::search(&config.query.as_str(), content.as_str());
    for (line, value) in matches {
        println!("{}: {}", line, value)
    }

    Ok(())
}

fn search(query: &str, contents: &str) -> LinkedHashMap<usize, String> {
    let mut results = LinkedHashMap::new();
    let mut line_num: usize = 1;
    for line in contents.lines() {
        if line.contains(query) {
            results.insert(line_num, line.to_string());
        }
        line_num += 1;
    }

    if results.len() != 0 {
        results
    } else {
        print_stderr(format!("Pattern '{}' is not present in file.", query)).unwrap();
        process::exit(1)
    }
}