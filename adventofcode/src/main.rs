use adventofcode::common::config::FileContentConfig;

use std::{env, process};

fn main() {
    let config = FileContentConfig::build(env::args()).unwrap_or_else(|err| {
        handle_parsing_error(err);
        process::exit(1);
    });

    if let Err(err) = adventofcode::day08::run(config) {
        handle_app_error(&err);
        process::exit(1);
    }
}

fn usage() {
    eprintln!("Usage: {} <file_name>", env::args().next().unwrap());
}

fn handle_parsing_error(err: &str) {
    perror(format!("Could not parse arguments: {}", err));
    usage();
}

fn handle_app_error(err: &str) {
    perror(format!("Application error: {}", err));
}

fn perror(err: String) {
    eprintln!("\n[\x1b[31mE\x1b[0m] {}", err);
}

