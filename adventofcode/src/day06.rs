use std::collections::VecDeque;

use crate::common::config::FileContentConfig;

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 6!");
    let res1 = part1_solve(&config.content)?;
    println!(
        "  Part 1: packet start index is {}",
        match res1 {
            Some(index) => index.to_string(),
            None => "not found".to_string(),
        }
    );

    let res2 = part2_solve(&config.content)?;
    println!(
        "  Part 2: message start index is {}",
        match res2 {
            Some(index) => index.to_string(),
            None => "not found".to_string(),
        }
    );

    return Ok(());
}

fn part1_solve(content: &str) -> Result<Option<usize>, String> {
    return solve(content, 4);
}

fn part2_solve(content: &str) -> Result<Option<usize>, String> {
    return solve(content, 14);
}

fn solve(content: &str, expected_sequence_size: usize) -> Result<Option<usize>, String> {
    // new elements at the back, old elements at the front
    let mut unique_sequence: VecDeque<char> = VecDeque::with_capacity(expected_sequence_size);

    for (i, ch) in content.chars().enumerate() {
        if unique_sequence.len() == expected_sequence_size {
            return Ok(Some(i));
        }

        let pos_option = unique_sequence.iter().position(|seq_ch| &ch == seq_ch);
        if let Some(position) = pos_option {
            unique_sequence = unique_sequence.split_off(position + 1);
        }

        unique_sequence.push_back(ch);
    }

    return Ok(None);
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Display};

    #[test]
    fn part1_examples() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Ok(Some(7))),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Ok(Some(5))),
            ("nppdvjthqldpwncqszvftbrmjlhg", Ok(Some(6))),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Ok(Some(10))),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Ok(Some(11))),
        ];

        for (input, expected) in examples {
            run_test(input, expected, super::part1_solve);
        }
    }

    #[test]
    fn part2_examples() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Ok(Some(19))),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Ok(Some(23))),
            ("nppdvjthqldpwncqszvftbrmjlhg", Ok(Some(23))),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Ok(Some(29))),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Ok(Some(26))),
        ];

        for (input, expected) in examples {
            run_test(input, expected, super::part2_solve);
        }
    }

    fn run_test<I, O, F>(input: &I, expected_output: O, fn_to_test: F)
    where
        I: Display + ?Sized,
        O: Debug + PartialEq,
        F: Fn(&I) -> O,
    {
        let output = fn_to_test(input);

        assert_eq!(
            output, expected_output,
            "unexpected result un input '{}', expected {:?} but got {:?}",
            input, expected_output, output
        );
    }
}

