use std::fmt::Display;

pub struct IllegalCharacter;

impl Display for IllegalCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown direction character")
    }
}

impl From<IllegalCharacter> for String {
    fn from(value: IllegalCharacter) -> Self {
        value.to_string()
    }
}

pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "U",
            Direction::Left => "L",
            Direction::Down => "D",
            Direction::Right => "R",
        })
    }
}

impl TryFrom<&u8> for Direction {
    type Error = IllegalCharacter;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'U' => Ok(Direction::Up),
            b'L' => Ok(Direction::Left),
            b'D' => Ok(Direction::Down),
            b'R' => Ok(Direction::Right),
            _ => Err(IllegalCharacter),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Default for Point {
    fn default() -> Self {
        return Point::new(0, 0);
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn translate(&mut self, offset_x: isize, offset_y: isize) {
        self.x += offset_x;
        self.y += offset_y;
    }
}

pub trait Rope {
    fn step(&mut self, direction: &Direction);

    fn tail(&self) -> &Point;
}

/// day 9 part 1
pub struct ShortRope {
    head: Point,
    tail: Point,
}

impl ShortRope {
    pub fn new() -> ShortRope {
        ShortRope {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
        }
    }
}

impl Rope for ShortRope {
    fn step(&mut self, direction: &Direction) {
        let (offset_x, offset_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        };
        self.head.translate(offset_x, offset_y);

        if offset_x != 0 {
            let tail_diff_x = self.head.x - self.tail.x;
            match tail_diff_x {
                2 => {
                    self.tail.translate(1, 0);
                    self.tail.y = self.head.y;
                }
                -2 => {
                    self.tail.translate(-1, 0);
                    self.tail.y = self.head.y;
                }
                _ => (),
            }
        } else if offset_y != 0 {
            let tail_diff_y = self.head.y - self.tail.y;
            match tail_diff_y {
                2 => {
                    self.tail.translate(0, 1);
                    self.tail.x = self.head.x;
                }
                -2 => {
                    self.tail.translate(0, -1);
                    self.tail.x = self.head.x;
                }
                _ => (),
            }
        }
    }

    fn tail(&self) -> &Point {
        &self.tail
    }
}

impl Display for ShortRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_offset_x = 0.min(self.head.x.min(self.tail.x));
        let origin_offset_y = 0.min(self.head.y.min(self.tail.y));

        let width = 1 - origin_offset_x + self.head.x.abs().max(self.tail.x.abs());
        let height = 1 - origin_offset_y + self.head.y.abs().max(self.tail.y.abs());
        // writeln!(f, "w,h=({}, {})", width, height)?;
        for rel_y in 0..height {
            let y = rel_y + origin_offset_y;

            for rel_x in 0..width {
                let x = rel_x + origin_offset_x;
                if x == self.head.x && y == self.head.y {
                    write!(f, "H")?;
                } else if x == self.tail.x && y == self.tail.y {
                    write!(f, "T")?;
                } else if y == 0 && x == 0 {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        return Ok(());
    }
}

// day 9 part 2

// struct RopeListNode {
//     knot: Point,
//     next: Option<Rc<RopeListNode>>,
// }
//
// impl RopeListNode {
//     fn new() -> RopeListNode {
//         return RopeListNode {
//             knot: Point::default(),
//             next: None,
//         };
//     }
// }

pub struct RopeList {
    knots: Vec<Point>,
    // head: Rc<RopeListNode>,
    // tail: Rc<RopeListNode>,
}

impl RopeList {
    pub fn new(size: usize) -> Self {
        assert!(size >= 2);

        return Self {
            knots: vec![Point::default(); size],
        };
    }

    pub fn to_grid_str(&self, size: usize) -> String {
        let mut grid_str = String::new();
        let min_coord = -(size as isize / 2);
        let max_coord = size as isize / 2;
        for y in min_coord..=max_coord {
            for x in min_coord..=max_coord {
                let point_opt = self.knots.iter().enumerate().find(|(_, point)| point.x == x && point.y == y);
                
                if let Some((index, _)) = point_opt {
                    grid_str += &index.to_string(); // can be too wide if rope size > 10
                } else if x == 0 && y == 0 {
                    grid_str += "s";
                } else {
                    grid_str += ".";
                }
            }
            grid_str += "\n";
        }
        return grid_str;
    }
}

impl Rope for RopeList {
    fn step(&mut self, direction: &Direction) {
        let (mut offset_x, mut offset_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        };

        let mut leader_opt: Option<&mut Point> = None;
        for follower in &mut self.knots {
            if let Some(leader) = leader_opt {
                leader.translate(offset_x, offset_y);
                let follower_diff_x = leader.x - follower.x;
                let follower_diff_y = leader.y - follower.y;

                if offset_x != 0 && offset_y != 0 && (follower_diff_x.abs() == 2 || follower_diff_y.abs() == 2) {
                    if follower_diff_x == 0 {
                        offset_x = 0;
                    }
                    if follower_diff_y == 0 {
                        offset_y = 0;
                    }
                } else if offset_x != 0 {
                    offset_y = leader.y - follower.y;
                    match follower_diff_x {
                        2 => {
                            offset_x = 1;
                        },
                        -2 => {
                            offset_x = -1;
                        },
                        _ => {
                            offset_x = 0;
                            offset_y = 0;
                        },
                    }
                } else if offset_y != 0 {
                    offset_x = leader.x - follower.x;
                    match follower_diff_y {
                        2 => {
                            offset_y = 1;
                        },
                        -2 => {
                            offset_y = -1;
                        },
                        _ => {
                            offset_x = 0;
                            offset_y = 0;
                        },
                    }
                }
                // println!("leader={:?} \tfollower={:?} \tfollower_offset=({}, {})", leader, follower, offset_x, offset_y);
            }
            leader_opt = Some(follower);
        }
        leader_opt.unwrap().translate(offset_x, offset_y);

        // println!("=== {} ===\n{}", direction, self.to_grid_str(30));
    }

    fn tail(&self) -> &Point {
        return self
            .knots
            .last()
            .expect("rope list shouldn't be empty by construction");
    }
}
