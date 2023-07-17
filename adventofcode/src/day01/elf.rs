#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Elf {
    pub calories: usize,
    id: usize,
}

impl Elf {
    pub fn new(id: usize, calories: usize) -> Self {
        return Elf {
            calories,
            id,
        }
    }
}

pub fn parse_elves<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();

    let mut current_total = 0;

    for line in input {
        if line.is_empty() { // exactly one empty line after each elf's data
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
    return &calories[calories.len()-result_count..];
}
