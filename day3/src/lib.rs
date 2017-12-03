pub mod part1;
pub mod part2;

pub const PUZZLE_INPUT: usize = 289326;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Displacement {
    north: i32,
    east: i32,
}

impl Displacement {
    fn new() -> Displacement {
        Displacement { north: 0, east: 0 }
    }

    fn manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }

    fn step(&self, direction: &Direction) -> Displacement {
        match *direction {
            Direction::North => Displacement {
                north: self.north + 1,
                east: self.east,
            },
            Direction::East => Displacement {
                north: self.north,
                east: self.east + 1,
            },
            Direction::South => Displacement {
                north: self.north - 1,
                east: self.east,
            },
            Direction::West => Displacement {
                north: self.north,
                east: self.east - 1,
            },
        }
    }

    fn neighbours(&self) -> Vec<Displacement> {
        vec![
            Displacement {
                north: self.north + 1,
                east: self.east,
            },
            Displacement {
                north: self.north + 1,
                east: self.east + 1,
            },
            Displacement {
                north: self.north,
                east: self.east + 1,
            },
            Displacement {
                north: self.north - 1,
                east: self.east + 1,
            },
            Displacement {
                north: self.north - 1,
                east: self.east,
            },
            Displacement {
                north: self.north - 1,
                east: self.east - 1,
            },
            Displacement {
                north: self.north,
                east: self.east - 1,
            },
            Displacement {
                north: self.north + 1,
                east: self.east - 1
            },
        ]
    }
}
