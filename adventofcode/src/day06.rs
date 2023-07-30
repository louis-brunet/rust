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
    // let res2 = part2_solve(&config.content)?;
    // println!("  Part 2: message={:?}", res2);
    return Ok(());
}

fn part1_solve(content: &str) -> Result<Option<usize>, String> {
    let expected_sequence_size = 4;
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
    use super::part1_solve;

    #[test]
    fn part1_examples() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in examples {
            let res = part1_solve(input)
                .unwrap_or_else(|_| panic!("unexpected error on input '{}'", input));

            assert_eq!(res, Some(expected), "unexpected result un input '{}', expected {:?} but got {:?}", input, Some(expected), res);
        }
    }
}
