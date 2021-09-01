use std::{env, process};

use pgrep;
use pgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        pgrep::util::stderr(format!("Problem parsing arguments: {}", err).as_str()).unwrap();
        process::exit(1);
    });

    pgrep::run(config).expect("Failed to run pgrep!");
}
