use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum InfectionState {
    Clean,
    Infected,
}

impl InfectionState {
    pub fn invert(&mut self) {
        *self = match *self {
            InfectionState::Clean => InfectionState::Infected,
            InfectionState::Infected => InfectionState::Clean,
        }
    }
}

struct InfiniteGrid {
    grid: HashMap<(i32, i32), InfectionState>,
}

impl InfiniteGrid {
    pub fn new() -> InfiniteGrid {
        InfiniteGrid {
            grid: HashMap::new(),
        }
    }
}

impl Index<(i32, i32)> for InfiniteGrid {
    type Output = InfectionState;

    fn index<'a>(&'a self, index: (i32, i32)) -> &'a Self::Output {
        self.grid.get(&index).unwrap_or(&InfectionState::Clean)
    }
}

impl IndexMut<(i32, i32)> for InfiniteGrid {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.grid.entry(index).or_insert(InfectionState::Clean)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

fn infect(grid: &mut InfiniteGrid, iterations: usize) -> usize {
    let mut infections: usize = 0;
    let mut position = (0, 0);
    let mut direction = Direction::North;

    for _ in 0..iterations {
        let current_sector = &mut grid[position];
        direction = match *current_sector {
            InfectionState::Clean => {
                infections += 1;
                direction.left()
            }
            InfectionState::Infected => direction.right(),
        };

        current_sector.invert();

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
    fn test_invert_sector() {
        let mut x = InfectionState::Clean;
        x.invert();
        assert_eq!(x, InfectionState::Infected);
        x.invert();
        assert_eq!(x, InfectionState::Clean);
    }

    #[test]
    fn part_1_example_a() {
        let mut grid = InfiniteGrid::new();
        grid[(1, 1)] = InfectionState::Infected;
        grid[(-1, 0)] = InfectionState::Infected;
        let infections = infect(&mut grid, 70);
        assert_eq!(infections, 41);
    }

    #[test]
    fn part_1_example_b() {
        let mut grid = InfiniteGrid::new();
        grid[(1, 1)] = InfectionState::Infected;
        grid[(-1, 0)] = InfectionState::Infected;
        let infections = infect(&mut grid, 10_000);
        assert_eq!(infections, 5587);
    }
}
