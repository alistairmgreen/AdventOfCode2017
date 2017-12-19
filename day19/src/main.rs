fn main() {
    let maze = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.chars().map(Element::from).collect::<Vec<Element>>())
        .collect::<Vec<Vec<Element>>>();

    let visited = follow(&maze);
    println!("The following points were visited:");
    for point in &visited {
        print!("{}", point);
    }
    println!();
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Element {
    Corner,
    Letter(char),
    Path,
    Space,
}

impl From<char> for Element {
    fn from(c: char) -> Element {
        match c {
            '+' => Element::Corner,
            '-' | '|' => Element::Path,
            'A'...'Z' => Element::Letter(c),
            _ => Element::Space,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

fn step(
    row: usize,
    column: usize,
    last_row: usize,
    last_column: usize,
    direction: &Direction,
) -> Option<(usize, usize)> {
    match *direction {
        Direction::North => if row > 0 {
            Some((row - 1, column))
        } else {
            None
        },
        Direction::East => if column < last_column {
            Some((row, column + 1))
        } else {
            None
        },
        Direction::South => if row < last_row {
            Some((row + 1, column))
        } else {
            None
        },
        Direction::West => if column > 0 {
            Some((row, column - 1))
        } else {
            None
        },
    }
}

fn follow(grid: &[Vec<Element>]) -> Vec<char> {
    let mut visited: Vec<char> = Vec::new();
    let mut direction = Direction::South;
    let mut row: usize = 0;
    let last_row = grid.len() - 1;
    let mut column: usize = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, x)| *x == Element::Path)
        .map(|(c, _)| c)
        .unwrap();

    loop {
        let last_column = grid[row].len() - 1;
        match step(row, column, last_row, last_column, &direction) {
            Some((r, c)) => {
                row = r;
                column = c;

                match grid[row][column] {
                    Element::Space => break,
                    Element::Letter(l) => visited.push(l),
                    Element::Path => {} // Just keep going
                    Element::Corner => {
                        // We have to turn left or right, whichever keeps us on the path.
                        let left = direction.left();
                        let right = direction.right();

                        if let Some((x, y)) = step(row, column, last_row, last_column, &left) {
                            if grid[x][y] != Element::Space {
                                direction = left;
                                continue;
                            }
                        }

                        if let Some((x, y)) = step(row, column, last_row, last_column, &right) {
                            if grid[x][y] != Element::Space {
                                direction = right;
                            }
                        }
                    }
                }
            }
            None => break,
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_from_letter() {
        assert_eq!(Element::from('P'), Element::Letter('P'));
    }

    #[test]
    fn element_from_space() {
        assert_eq!(Element::from(' '), Element::Space);
    }

    #[test]
    fn example_path() {
        let maze =
"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

        let elements = maze.lines()
            .map(|line| line.chars().map(Element::from).collect::<Vec<Element>>())
            .collect::<Vec<Vec<Element>>>();

        let visited = follow(&elements);

        assert_eq!(visited, vec!['A', 'B', 'C', 'D', 'E', 'F']);
    }
}
