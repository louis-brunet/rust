#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Elf {
    pub calories: usize,
    id: usize,
}

impl Elf {
    pub fn new(id: usize, calories: usize) -> Self {
        return Elf { calories, id };
    }
}

pub fn parse_elves<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();

    let mut current_total = 0;

    for line in input {
        if line.is_empty() {
            // exactly one empty line after each elf's data
            elves.push(Elf::new(elves.len(), current_total));
            current_total = 0;
        } else {
            current_total += line.parse::<usize>().expect("should parse int");
        }
    }

    return elves;
}

pub fn get_highest_calorie_elves(calories: &mut Vec<Elf>, result_count: usize) -> &[Elf] {
    // TODO don't sort the whole list, try to get the `result_count` maximum values
    calories.sort();
    return &calories[calories.len() - result_count..];
}

#[cfg(test)]
mod test {
    use crate::day01::elf::{parse_elves, Elf};

    #[test]
    fn create_elf() {
        let elf = Elf::new(0, 0);
        assert_eq!(elf.calories, 0);
    }

    #[test]
    fn parse() {
        let elves = parse_elves(["10", "123446", ""].into_iter());
        assert_eq!(1, elves.len());
        assert_eq!(123456, elves[0].calories);
    }
}
