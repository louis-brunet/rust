mod config;
mod elf;

use std::fs;
pub use config::Config;

/// Solve day 1 https://adventofcode.com/2022/day/1
pub fn run(config: Config) -> Result<(), &'static str> {
    println!("Hello from day01! {:?}", config);

    let contents = fs::read_to_string(&config.file_path);
    if let Err(_) = contents {
        return Err("could not open and read file {}");
    }

    let mut elves = elf::parse_elves(contents.unwrap().lines());
    let count = 3;
    let top_elves = elf::get_highest_calorie_elves(&mut elves, count);
    dbg!(top_elves);
    let mut total = 0;
    for e in top_elves {
        total += e.calories;
    }
    dbg!(total);

    return Ok(());
}
