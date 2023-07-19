use crate::common::config::FileContentConfig;
use crate::day02::rock_paper_scissors::{RoundOutcome, RoundShapes, Shape};

mod rock_paper_scissors;

/// Solve day 2 https://adventofcode.com/2022/day/2
pub fn solve(config: FileContentConfig) -> Result<(), &'static str> {
    println!("Day 2!");
    // println!("{:?}", config);

    let lines = config.content.lines();
    let moves = parse_moves_part1(lines);
    let score_part1 = get_score(moves);
    println!("Expected score for part 1: {}", score_part1);

    let lines = config.content.lines();
    let moves = parse_moves_part2(lines);
    let score_part2 = get_score(moves);
    println!("Expected score for part 2: {}", score_part2);

    return Ok(());
}

fn get_score(moves: impl Iterator<Item = Result<RoundShapes, &'static str>>) -> i32 {
    let mut score = 0;

    for round_shapes in moves {
        let round_shapes = round_shapes.unwrap();
        let outcome = round_shapes.my_shape.outcome(&round_shapes.opponent_shape);
        score += match outcome {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            RoundOutcome::Lose => 0,
        };
        score += match round_shapes.my_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
    }

    return score;
}

fn parse_moves_part1<'a>(
    input: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = Result<RoundShapes, &'static str>> + 'a {
    return input.map(|line| {
        let mut split = line.split(" ");
        let opponent_char: Option<char> = match split.next() {
            None => return Err("expected round outcome"),
            Some(shape_str) => shape_str.chars().next(),
        };
        let opponent_shape = match opponent_char {
            Some('A') => Shape::Rock,
            Some('B') => Shape::Paper,
            Some('C') => Shape::Scissors,
            _ => return Err("expected opponent shape"),
        };
        let my_char: Option<char> = match split.next() {
            None => return Err("expected round outcome"),
            Some(shape_str) => shape_str.chars().next(),
        };
        let my_shape = match my_char {
            Some('X') => Shape::Rock,
            Some('Y') => Shape::Paper,
            Some('Z') => Shape::Scissors,
            _ => return Err("expected player shape"),
        };

        return Ok(RoundShapes {
            opponent_shape,
            my_shape,
        });
    });
}

fn parse_moves_part2<'a>(
    input: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = Result<RoundShapes, &'static str>> + 'a {
    return input.map(|line| {
        let mut split = line.split(" ");
        let opponent_char: Option<char> = match split.next() {
            None => return Err("expected round outcome"),
            Some(shape_str) => shape_str.chars().next(),
        };
        let opponent_shape = match opponent_char {
            Some('A') => Shape::Rock,
            Some('B') => Shape::Paper,
            Some('C') => Shape::Scissors,
            _ => return Err("expected opponent shape"),
        };
        let outcome_char: Option<char> = match split.next() {
            None => return Err("expected round outcome"),
            Some(shape_str) => shape_str.chars().next(),
        };
        let shapes = [Shape::Rock, Shape::Paper, Shape::Scissors];
        let my_shape = match outcome_char {
            Some('X') => shapes
                .into_iter()
                .find(|me| me.outcome(&opponent_shape) == RoundOutcome::Lose)
                .unwrap(),
            Some('Y') => shapes
                .into_iter()
                .find(|me| me.outcome(&opponent_shape) == RoundOutcome::Draw)
                .unwrap(),
            Some('Z') => shapes
                .into_iter()
                .find(|me| me.outcome(&opponent_shape) == RoundOutcome::Win)
                .unwrap(),
            _ => return Err("expected round outcome"),
        };

        return Ok(RoundShapes {
            opponent_shape,
            my_shape,
        });
    });
}

#[cfg(test)]
mod test {
    use crate::day02::{get_score, parse_moves_part1, parse_moves_part2};

    #[test]
    fn part1_example() {
        let input = String::from("A Y\nB X\nC Z\n");
        let lines = input.lines();
        let moves = parse_moves_part1(lines);
        assert_eq!(15, get_score(moves));
    }

    #[test]
    fn part2_example() {
        let input = String::from("A Y\nB X\nC Z\n");
        let lines = input.lines();
        let moves = parse_moves_part2(lines);
        assert_eq!(12, get_score(moves));
    }
}
