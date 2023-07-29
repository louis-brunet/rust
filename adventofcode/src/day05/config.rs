#[derive(Debug, PartialEq)]
pub struct Crate {
    pub label: char,
}

impl Crate {
    pub fn new(label: char) -> Self {
        return Self { label };
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    pub(crate) fn new(count: usize, from: usize, to: usize) -> Instruction {
        return Instruction { count, from, to };
    }
}

