use adventofcode::common::config::FileContentConfig;

use std::{env, process};

fn main() {
    let config = FileContentConfig::build(env::args()).unwrap_or_else(|err| {
        handle_parsing_error(err);
        process::exit(1);
    });

    if let Err(err) = adventofcode::day02::solve(config) {
        handle_app_error(err);
        process::exit(1);
    }
}

fn usage() {
    eprintln!("Usage: {}", env::args().next().unwrap())
}

fn handle_parsing_error(err: &str) {
    eprintln!("Could not parse arguments: {err}");
    usage();
}

fn handle_app_error(err: &str) {
    eprintln!("Application error: {}", err);
}
