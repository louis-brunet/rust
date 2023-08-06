mod config;
mod lexer;
mod parser;

use std::collections::VecDeque;

use crate::common::config::FileContentConfig;

use self::{lexer::Lexer, config::{Instruction, Crate}};

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 5!");
    let res1 = part1_solve(&config.content)?;
    println!("  Part 1: message={:?}", res1);
    let res2 = part2_solve(&config.content)?;
    println!("  Part 2: message={:?}", res2);
    return Ok(());
}

fn part1_solve(input: &str) -> Result<String, String> {
    let (mut stacks, instructions) = match parser::parse_part1(&mut Lexer::new(input)) {
        Ok(parsed) => parsed,
        Err(err) => return Err(format!("parser error: {}", err)),
    };

    execute_instructions_part1(&mut stacks, &instructions);

    let mut message = String::new();
    for mut stack in stacks {
        match stack.pop_front() {
            Some(cr) => message.push_str(&cr.label.to_string()),
            None => (),
        }
    }

    return Ok(message);
}

fn part2_solve(input: &str) -> Result<String, String> {
    let (mut stacks, instructions) = match parser::parse_part1(&mut Lexer::new(input)) {
        Ok(parsed) => parsed,
        Err(err) => return Err(format!("parser error: {}", err)),
    };

    execute_instructions_part2(&mut stacks, &instructions);

    let mut message = String::new();
    for mut stack in stacks {
        match stack.pop_front() {
            Some(cr) => message.push_str(&cr.label.to_string()),
            None => (),
        }
    }

    return Ok(message);
}

fn execute_instructions_part1(stacks: &mut [VecDeque<Crate>], instructions: &Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let picked_up = stacks[instruction.from].pop_front();

            match picked_up {
                Some(cr) => stacks[instruction.to].push_front(cr),
                None => (),
            }
        }
    }
}

fn execute_instructions_part2(stacks: &mut [VecDeque<Crate>], instructions: &Vec<Instruction>) {
    let mut picked_up: Vec<Crate> = Vec::new();
    for instruction in instructions {
        // picked_up.clear();

        for _ in 0..instruction.count {
            match stacks[instruction.from].pop_front() {
                Some(cr) => picked_up.push(cr),
                None => (),
            }
        }

        while let Some(cr) = picked_up.pop() {
            stacks[instruction.to].push_front(cr);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day05;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";

    /* #[test]
    fn part1_first_example() {
        let input = "\
        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let expected = "CMZ";
        let result = day05::part1_solve(input).expect("test error");

        assert_eq!(expected, result);
    } */

    #[test]
    fn part1_example() {
        let expected = "CMZ";
        let result = day05::part1_solve(INPUT).expect("test error");

        assert_eq!(expected, result);
    }

    #[test]
    fn part2_example() {
        let expected = "MCD";
        let result = day05::part2_solve(INPUT).expect("test error");

        assert_eq!(expected, result);
    }
}
