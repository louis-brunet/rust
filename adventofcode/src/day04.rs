mod lexer;

use std::fmt::Display;

use crate::common::config::FileContentConfig;

use self::lexer::{Lexer, Token};

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 4!");
    let res = part1_solve(&config.content)?;
    println!(" res = {}", res);
    return Ok(());
}

#[derive(Debug)]
pub struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        return self.min <= other.min && self.max >= other.max;
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}-{}", self.min, self.max);
    }
}

fn part1_solve<'a>(input: &str) -> Result<usize, String> {
    let mut contained_count = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let range1 = match parse_range(&mut lexer)? {
            None => return Ok(contained_count),
            Some(r) => r,
        };

        match lexer.get_next_token() {
            Ok((Token::Comma, ..)) => (),
            Ok((tk, ..)) => return Err(format!("Unexpected token: {:?} ", tk)),
            Err(err) => return Err(err.to_string()),
        };

        let range2 = match parse_range(&mut lexer)? {
            None => return Err(String::from("unexpected EOF")),
            Some(r) => r,
        };

        match lexer.get_next_token() {
            Ok((Token::NewLine, ..)) => (),
            Ok((tk, ..)) => return Err(format!("Unexpected token: {:?} ", tk)),
            Err(err) => return Err(err.to_string()),
        };

        if range1.contains(&range2) || range2.contains(&range1) {
            contained_count += 1;
        }
    }
}

fn parse_range(lexer: &mut Lexer) -> Result<Option<Range>, String> {
    let mut min_token = Token::NewLine;
    while let Token::NewLine = min_token {
        min_token =  match lexer.get_next_token() {
            Ok((tk @ Token::Number(_), ..)) => tk,
            Ok((tk @ Token::NewLine, ..)) => tk,
            Ok((Token::EOF, ..)) => return Ok(None),
            Ok((tk, ..)) => return Err(format!("Unexpected token: {:?} ", tk)),
            Err(err) => return Err(err.to_string()),
        };
    }
    
    let min = match min_token {
        Token::Number(n) => n,
        tk => return Err(format!("Unexpected token: {:?} ", tk)),
    };
    match lexer.get_next_token() {
        Ok((Token::Dash, ..)) => (),
        Ok((tk, ..)) => return Err(format!("Unexpected token: {:?} ", tk)),
        Err(err) => return Err(err.to_string()),
    };
    let max = match lexer.get_next_token() {
        Ok((Token::Number(n), ..)) => n,
        Ok((tk, _, _)) => return Err(format!("Unexpected token: {:?} ", tk)),
        Err(err) => return Err(err.to_string()),
    };

    return Ok(Some(Range {
        min: min.try_into().unwrap(),
        max: max.try_into().unwrap(),
    }));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    pub fn part1_example() {
        let expected = 2;

        let res = super::part1_solve(INPUT).unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    pub fn part1_multidigit() {
        let expected = 3;
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
22-66,44-88
45-66,44-88
";
        let res = super::part1_solve(input).unwrap();

        assert_eq!(res, expected);
    }
}
