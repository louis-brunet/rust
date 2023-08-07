//! https://adventofcode.com/2022/day/8

use std::fmt::Display;

use crate::common::config::FileContentConfig;

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 8!");

    let visible_count = part1_solve(&config.content)?;
    println!("{} visible trees", visible_count);

    let highest_score = part2_solve(&config.content)?;
    println!("highest scenic score: {}", highest_score);

    Ok(())
}

fn parse_grid(content: &str) -> Result<Vec<Vec<u8>>, ForestParserError> {
    let width = match content.lines().next() {
        Some(line) => line.len(),
        _ => return Err(ForestParserError::NoInput),
    };
    if width == 0 {
        return Err(ForestParserError::InvalidRowSize(0, width));
    }

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            break;
        }

        let mut row: Vec<u8> = Vec::with_capacity(width);
        for ch in line.bytes() {
            if !ch.is_ascii_digit() {
                return Err(ForestParserError::InvalidHeight(ch as char));
            }
            row.push(ch - b'0');
        }

        if row.len() != width {
            return Err(ForestParserError::InvalidRowSize(grid.len(), row.len()));
        }

        grid.push(row);
    }

    Ok(grid)
}

fn part1_solve(input: &str) -> Result<usize, String> {
    let grid = parse_grid(input)?;
    
    println!("grid is {}x{}", grid[0].len(), grid.len());
    let mut visible_grid: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();
    visible_grid[0] = vec![true; visible_grid[0].len()];
    let mut top_max_height = grid[0].clone();

    for row_idx in 1..grid.len() {
        let row = &grid[row_idx];
        let mut left_max_height = row[0];
        visible_grid[row_idx][0] = true;
        
        for col_idx in 1..row.len() {
            let tree_height = row[col_idx];
            if tree_height > top_max_height[col_idx] {
                visible_grid[row_idx][col_idx] = true;
                top_max_height[col_idx] = tree_height;
            }
            if tree_height > left_max_height {
                visible_grid[row_idx][col_idx] = true;
                left_max_height = tree_height;
            }
        }

        let &(mut right_max_height) = row.last().expect("row shouldn't be empty");
        visible_grid[row_idx][row.len() - 1] = true;
        
        for col_idx in  (1..row.len() - 1).rev() {
            // if visible_grid[row_idx][col_idx] {
            //     break;
            // }
            let tree_height = row[col_idx];
            if tree_height > right_max_height {
                right_max_height = tree_height;
                visible_grid[row_idx][col_idx] = true;
            }
        }
    }
    
    let last_row = grid.last().expect("grid shouldn't be empty");
    for col_idx in (1..last_row.len()).rev() {
        visible_grid.last_mut().unwrap()[col_idx] = true;
        let mut bottom_max_height = last_row[col_idx];

        for row_idx in (1..grid.len() - 1).rev() {
            // if visible_grid[row_idx][col_idx] {
            //     break;
            // }
            let tree_height = grid[row_idx][col_idx];
            if tree_height > bottom_max_height {
                bottom_max_height = tree_height;
                visible_grid[row_idx][col_idx] = true;
            }
        }
    }


    let mut visible_count = 0;
    for row in visible_grid {
        for is_visible in row {
            if is_visible {
                print!("|");
                // println!("({}, {}) is visible", );
                visible_count += 1;
            } else {
                print!("_");
            }
        }
        println!();
    }

    Ok(visible_count)
}

fn part2_solve(input: &str) -> Result<usize, String> {
    let grid = parse_grid(input)?;

    let mut scenic_score: Vec<Vec<usize>> = grid.iter().map(|row| vec![0; row.len()]).collect();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, tree_height) in row.iter().enumerate() {
            if row_idx == 0 || row_idx == grid.len() - 1 || col_idx == 0 || col_idx == row.len() - 1 {
                continue;
            }

            let mut visible_top = 0;
            for r in grid[..row_idx].iter().rev() {
                visible_top += 1;
                if &r[col_idx] >= tree_height {
                    break;
                }
            }

            let mut visible_bottom = 0;
            for r in grid[row_idx+1..].iter() {
                visible_bottom += 1;
                if &r[col_idx] >= tree_height {
                    break;
                }
            }

            let mut visible_right = 0;
            for other_height in row[col_idx + 1..].iter() {
                visible_right += 1;
                if other_height >= tree_height {
                    break;
                }
            }

            let mut visible_left = 0;
            for other_height in row[..col_idx].iter().rev() {
                visible_left += 1;
                if other_height >= tree_height {
                    break;
                }
            }

            scenic_score[row_idx][col_idx] = visible_left * visible_right * visible_top * visible_bottom;
            // print!("[{};{};{};{}]{} ", visible_top, visible_bottom, visible_right, visible_left, scenic_score[row_idx][col_idx]);
        }
        // println!()
    }

    let max = scenic_score.iter().flatten().max().expect("there should be data");
    Ok(*max)
}

enum ForestParserError {
    NoInput,
    InvalidHeight(char),
    InvalidRowSize(usize, usize), // line number, size
}

impl From<ForestParserError> for String {
    fn from(value: ForestParserError) -> Self {
        value.to_string()
    }
}

impl Display for ForestParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ForestParserError::*;

        write!(f, "{}", match self {
            InvalidHeight(ch) => format!("invalid height \"{}\"", ch),
            NoInput => "input is empty".to_string(),
            InvalidRowSize(line, size) => format!("line {} has an invalid size: {}", line, size),
        })
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn part1_example() {
        let res = super::part1_solve(EXAMPLE_INPUT).unwrap();

        assert_eq!(res, 21);
    }

    #[test]
    fn part1() {
        let input = "\
9999
2139
9999
9999
";
        assert_eq!(super::part1_solve(input).unwrap(), 13);

        let input = "\
9999
9312
9999
9999
";
        assert_eq!(super::part1_solve(input).unwrap(), 13);

        let input = "\
9999
9399
9199
9299
";
        assert_eq!(super::part1_solve(input).unwrap(), 13);

        let input = "\
9299
9199
9399
9999
";
        assert_eq!(super::part1_solve(input).unwrap(), 13);

        let input = "\
99999
95999
13999
91999
91999
";
        assert_eq!(super::part1_solve(input).unwrap(), 19);

        let input = "\
99199
99199
95312
99999
99999
";
        assert_eq!(super::part1_solve(input).unwrap(), 19);
    }

    #[test]
    fn part2_example() {
        let res = super::part2_solve(EXAMPLE_INPUT).unwrap();

        assert_eq!(res, 8);
    }
}
