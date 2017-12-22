use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::{FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum InfectionState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl From<char> for InfectionState {
   fn from(c: char) -> InfectionState {
       match c {
           '#' => InfectionState::Infected,
           _ => InfectionState::Clean
       }
   } 
}

#[derive(Debug, Default)]
pub struct InfiniteGrid {
    grid: HashMap<(i32, i32), InfectionState>,
}

impl InfiniteGrid {
    pub fn new() -> InfiniteGrid {
        InfiniteGrid {
            grid: HashMap::new(),
        }
    }
}

impl FromStr for InfiniteGrid {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = InfiniteGrid::new();
        let lines = s.lines().collect::<Vec<&str>>();
        let middle_line = lines.len() as i32 / 2;

        for (row, line) in lines.iter().enumerate() {
            let y = middle_line - (row as i32);
            let middle_column = line.len() as i32 / 2;
            for (column, character) in line.chars().enumerate() {
                let x = (column as i32) - middle_column;
                grid[(x, y)] = character.into();
            }
        }

        Ok(grid)
    }
}

impl Index<(i32, i32)> for InfiniteGrid {
    type Output = InfectionState;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.grid.get(&index).unwrap_or(&InfectionState::Clean)
    }
}

impl IndexMut<(i32, i32)> for InfiniteGrid {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.grid.entry(index).or_insert(InfectionState::Clean)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn reverse(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn step(position: &(i32, i32), direction: Direction) -> (i32, i32) {
    let &(x, y) = position;

    match direction {
        Direction::North => (x, y + 1),
        Direction::East => (x + 1, y),
        Direction::South => (x, y - 1),
        Direction::West => (x - 1, y),
    }
}

pub fn infect(grid: &mut InfiniteGrid, iterations: usize) -> usize {
    let mut infections: usize = 0;
    let mut position = (0, 0);
    let mut direction = Direction::North;

    for _ in 0..iterations {
        let current_sector = &mut grid[position];

        direction = match *current_sector {
            InfectionState::Clean => direction.left(),
            InfectionState::Weakened => direction,
            InfectionState::Infected => direction.right(),
            InfectionState::Flagged => direction.reverse()
        };

        *current_sector = match *current_sector {
            InfectionState::Clean => InfectionState::Weakened,
            InfectionState::Weakened => InfectionState::Infected,
            InfectionState::Infected => InfectionState::Flagged,
            InfectionState::Flagged => InfectionState::Clean
        };

        if *current_sector == InfectionState::Infected {
            infections += 1;
        }

        position = step(&position, direction);
    }

    infections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite_grid_defaults_to_clean() {
        let grid = InfiniteGrid::new();
        assert_eq!(grid[(1, -2)], InfectionState::Clean);
    }

    #[test]
    fn set_entry_in_infinite_grid() {
        let mut grid = InfiniteGrid::new();
        grid[(1, 2)] = InfectionState::Infected;

        assert_eq!(grid[(1, 2)], InfectionState::Infected);
    }

    #[test]
    fn part_2_example_a() {
        let mut grid: InfiniteGrid = "..#\n#..\n...".parse().unwrap();
        println!("Grid: {:?}", grid);
        let infections = infect(&mut grid, 100);
        assert_eq!(infections, 26);
    }

    #[test]
    fn part_2_example_b() {
        let mut grid: InfiniteGrid = "..#\n#..\n...".parse().unwrap();
        let infections = infect(&mut grid, 10_000_000);
        assert_eq!(infections, 2_511_944);
    }
}
