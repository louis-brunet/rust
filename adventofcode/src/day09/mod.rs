//! https://adventofcode.com/2022/day/9

mod rope;

use std::collections::HashSet;

use crate::{
    common::config::FileContentConfig,
    day09::rope::{Point, ShortRope, RopeList},
};

use self::rope::Rope;

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 9!");
    let res1 = part1_solve(&config.content)?;
    println!("  Part 1: position count = {}", res1);
    let res2 = part2_solve(&config.content)?;
    println!("  Part 2: position count = {}", res2);
    return Ok(());
}

fn parse_and_step(input: &str, rope: &mut impl Rope) -> Result<usize, String> {
    let mut positions: HashSet<Point> = HashSet::new();

    // println!("{}", rope);
    for line in input.lines().take_while(|l| !l.is_empty()) {
        let bytes: Vec<u8> = line.bytes().collect();
        let direction = bytes
            .first()
            .expect("expected direction (U,D,L,R)")
            .try_into()?;
        let count = bytes
            .iter()
            .skip(2)
            .take_while(|ch| ch.is_ascii_digit())
            .fold(0, |acc, ch| acc * 10 + (ch - b'0'));
        // println!("\n=== {} {} ===", bytes[0] as char, count);
        for _ in 0..count {
            rope.step(&direction);
            positions.insert(rope.tail().clone());
            // println!("{}", rope);
        }
    }

    // println!("{}", positions_to_grid_str(&positions, 30));
    // println!("visited: {:?}", positions);
    Ok(positions.len())
}

fn part1_solve(input: &str) -> Result<usize, String> {
    let mut rope = ShortRope::new();
    return parse_and_step(input, &mut rope);
}

fn part2_solve(input: &str) -> Result<usize, String> {
    let mut rope = RopeList::new(10);
    let position_count = parse_and_step(input, &mut rope);
    // println!("{}", rope.to_grid_str(30));
    return position_count;
}

fn positions_to_grid_str(positions: &HashSet<Point>, size: usize) -> String {
    let mut grid_str = String::new();
    let min_coord = -(size as isize / 2);
    let max_coord = size as isize / 2;
    for y in min_coord..=max_coord {
        for x in min_coord..=max_coord {
            let point_opt = positions.iter().find(|point| *point == &Point::new(x, y));

            if x == 0 && y == 0 {
                grid_str += "s";
            } else if point_opt.is_some() {
                grid_str += "#";
            } else {
                grid_str += ".";
            }
        }
        grid_str += "\n";
    }
    return grid_str;
}

#[cfg(test)]
mod test {
    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn part1_example() {
        let res = super::part1_solve(INPUT).unwrap();

        assert_eq!(res, 13);
    }

    #[test]
    fn part2_example() {
        let res = super::part2_solve(INPUT).unwrap();

        assert_eq!(res, 1);
    }

    #[test]
    fn part2_large_example() {
        let large_input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        let res = super::part2_solve(large_input).unwrap();

        assert_eq!(res, 36);
    }
}
