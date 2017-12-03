use std::collections::HashSet;

fn main() {
    let puzzle_input: usize = 289326;
    println!("Data from square {} requires {} steps", puzzle_input, steps_required(puzzle_input));
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Displacement {
    north: i32,
    east: i32
}

impl Displacement {
    pub fn new() -> Displacement {
        Displacement { north: 0, east: 0 }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }

    pub fn step(&self, direction: &Direction) -> Displacement {
        match *direction {
            Direction::North => Displacement { north: self.north + 1, east: self.east },
            Direction::East => Displacement { north: self.north, east: self.east + 1 },
            Direction::South => Displacement { north: self.north - 1, east: self.east },
            Direction::West => Displacement { north: self.north, east: self.east - 1 }
        }
    }
}

fn steps_required(n: usize) -> i32 {
    let mut squares_visited: HashSet<Displacement> = HashSet::with_capacity(n);
    let mut position = Displacement::new();
    squares_visited.insert(position.clone());
    let mut direction = Direction::East;

    for _ in 2..n+1 {
        position = position.step(&direction);
        squares_visited.insert(position.clone());

        let left_direction = direction.turn_left();
        let square_to_my_left = position.step(&left_direction);
        if !squares_visited.contains(&square_to_my_left) {
            direction = left_direction;
        }
    }

    position.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_1_no_steps() {
        assert_eq!(steps_required(1), 0);
    }

    #[test]
    fn from_12_is_3_steps() {
        assert_eq!(steps_required(12), 3);        
    }

    #[test]
    fn from_23_is_2_steps() {
        assert_eq!(steps_required(23), 2);        
    }

    #[test]
    fn from_1024_is_31_steps() {
        assert_eq!(steps_required(1024), 31);        
    }
}