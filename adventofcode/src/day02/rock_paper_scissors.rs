#[derive(Debug, PartialEq)]
pub enum RoundOutcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub struct RoundShapes {
    pub opponent_shape: Shape,
    pub my_shape: Shape,
}

impl Shape {
    pub(crate) fn outcome(&self, other: &Self) -> RoundOutcome {
        return match self {
            Shape::Rock => match other {
                Shape::Rock => RoundOutcome::Draw,
                Shape::Paper => RoundOutcome::Lose,
                Shape::Scissors => RoundOutcome::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => RoundOutcome::Win,
                Shape::Paper => RoundOutcome::Draw,
                Shape::Scissors => RoundOutcome::Lose,
            },
            Shape::Scissors => match other {
                Shape::Rock => RoundOutcome::Lose,
                Shape::Paper => RoundOutcome::Win,
                Shape::Scissors => RoundOutcome::Draw,
            },
        };
    }
}

#[cfg(test)]
mod test {
    use super::{RoundOutcome, Shape};

    #[test]
    fn shape_outcome() {
        assert_eq!(RoundOutcome::Win, Shape::Rock.outcome(&Shape::Scissors));
        assert_eq!(RoundOutcome::Lose, Shape::Rock.outcome(&Shape::Paper));
        assert_eq!(RoundOutcome::Draw, Shape::Rock.outcome(&Shape::Rock));
    }
}
