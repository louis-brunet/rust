use crate::common::config::FileContentConfig;

pub fn run(config: FileContentConfig) -> Result<(), &'static str> {
    let res1 = part1_solve(config.content.lines());
    println!("Part 1 : sum = {:?}", res1);

    let res2 = part2_solve(config.content.lines());
    println!("Part 2 : sum = {:?}", res2);

    return Ok(());
}

fn part1_solve<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    let mut sum: usize = 0;

    'outer: for line in input {
        let split_index = line.len() / 2;

        for ch_left in line[..split_index].bytes() {
            for ch_right in line[split_index..].bytes() {
                if ch_left == ch_right {
                    // duplicate item in left and right
                    sum += get_item_prio(ch_left);

                    continue 'outer;
                }
            }
        }
    }
    return sum;
}

fn part2_solve<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    const GROUP_SIZE: usize = 3;
    let mut group: Vec<&str> = Vec::new();
    let mut sum: usize = 0;

    'outer: for line in input {
        if group.len() < GROUP_SIZE {
            group.push(line);
            if group.len() < GROUP_SIZE {
                continue;
            }
        }

        for ch1 in group[0].bytes() {
            for ch2 in group[1].bytes() {
                if ch1 != ch2 {
                    continue;
                }
                for ch3 in group[2].bytes() {
                    if ch2 == ch3 {
                        // duplicate item in all sacks
                        sum += get_item_prio(ch3);

                        group.clear();

                        continue 'outer;
                    }
                }
            }
        }

        group.clear();
    }

    return sum;
}

fn get_item_prio(item: u8) -> usize {
    let upper_bonus = if item.is_ascii_uppercase() { 26 } else { 0 };
    let ch_prio = upper_bonus + 1 + (item.to_ascii_lowercase() - 'a' as u8);
    // println!("Found duplicate '{}' prio is {}", ch3 as char, ch_prio);

    return ch_prio as usize;
}

#[cfg(test)]
mod test {
    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn part1_example() {
        let expected = 157;

        let res = super::part1_solve(INPUT.lines());

        assert_eq!(res, expected);
    }

    #[test]
    fn part2_example() {
        let expected = 70;

        let res = super::part2_solve(INPUT.lines());

        assert_eq!(res, expected);
    }
}
