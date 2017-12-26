#[macro_use]
extern crate failure;
use std::str::FromStr;
use failure::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Pixel {
    Off,
    On,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::Off
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Pixel {
        match c {
            '#' => Pixel::On,
            _ => Pixel::Off,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pixel::Off => write!(f, "."),
            Pixel::On => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Grid {
    elements: Vec<Pixel>,
    size: usize,
}

// impl<'a> Index<usize> for &'a Grid {
//     type Output = &'a [Pixel];

//     fn index(&'a self, index: usize) -> &'a [Pixel] {
//         let start = index * self.size;
//         let end = start + self.size;
//         let x = self.elements[start..end];

//         &x;
//     }
// }

impl Grid {
    fn row(&self, index: usize) -> &[Pixel] {
        let row_start = index * self.size;
        &self.elements[row_start..row_start + self.size]
    }

    fn partition(&self) -> Vec<Grid> {
        let partition_size = if self.size % 2 == 0 { 2 } else { 3 };
        let partition_element_count = partition_size * partition_size;
        let partitions_per_side = self.size / partition_size;
        let mut subgrids: Vec<Grid> = Vec::with_capacity(partitions_per_side * partitions_per_side);

        for y in 0..partitions_per_side {
            let y_offset = y * partition_size;

            for x in 0..partitions_per_side {
                let x_offset = x * partition_size;
                let mut partition_elements: Vec<Pixel> =
                    Vec::with_capacity(partition_element_count);
                for row in y_offset..y_offset + partition_size {
                    partition_elements
                        .extend_from_slice(&self.row(row)[x_offset..x_offset + partition_size]);
                }

                subgrids.push(Grid {
                    elements: partition_elements,
                    size: partition_size,
                });
            }
        }

        subgrids
    }

    fn compose(pieces: &[Grid]) -> Grid {
        let piece_size = pieces[0].size;
        let size = piece_size * piece_size;
        let mut elements: Vec<Pixel> = Vec::with_capacity(size * size);

        for row_pieces in pieces.chunks(piece_size) {
            for row in 0..piece_size {
                for piece in row_pieces {
                    elements.extend_from_slice(piece.row(row));
                }
            }
        }

        Grid {
            elements: elements,
            size: size
        }
    }

    fn flip_vertical(&self) -> Grid {
        let mut elements: Vec<Pixel> = Vec::with_capacity(self.elements.len());
        for row in (0..self.size).rev() {
            elements.extend_from_slice(self.row(row));
        }

        Grid {
            elements: elements,
            size: self.size
        }
    }

    fn flip_horizontal(&self) -> Grid {
        let mut elements: Vec<Pixel> = Vec::with_capacity(self.elements.len());
        for mut row in self.elements.chunks(self.size).map(|r| r.to_owned()) {
            row.reverse();
            elements.extend(row);
        }

        Grid {
            elements: elements,
            size: self.size
        }
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.split('/').collect();
        let size = rows.len();
        if size < 1 {
            bail!("Grid is empty.");
        }

        let mut elements: Vec<Pixel> = Vec::with_capacity(size * size);
        for row in rows {
            if row.len() != size {
                bail!("Grid is not square.");
            }
            elements.extend(row.chars().map(Pixel::from));
        }

        let grid = Grid {
            elements: elements,
            size: size,
        };

        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.size {
            for element in self.row(row) {
                write!(f, "{}", element)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_string() {
        let grid = Grid::from_str("../.#").unwrap();
        assert_eq!(grid.size, 2);
        assert_eq!(
            grid.elements,
            vec![Pixel::Off, Pixel::Off, Pixel::Off, Pixel::On]
        );
    }

    #[test]
    fn get_row_from_grid() {
        let grid = Grid::from_str("#..#/..../..../#..#").unwrap();
        assert_eq!(grid.row(0), &[Pixel::On, Pixel::Off, Pixel::Off, Pixel::On]);
        assert_eq!(grid.row(1), &[Pixel::Off, Pixel::Off, Pixel::Off, Pixel::Off]);
        assert_eq!(grid.row(2), &[Pixel::Off, Pixel::Off, Pixel::Off, Pixel::Off]);
        assert_eq!(grid.row(3), &[Pixel::On, Pixel::Off, Pixel::Off, Pixel::On]);
    }

    #[test]
    fn partition_3x3_grid() {
        let grid = Grid::from_str(".#./..#/###").unwrap();
        let parts = grid.partition();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0], grid);
    }

    #[test]
    fn partition_4x4_grid() {
        /*
            Grid pattern:
            #..#
            ....
            ....
            #..#
        */

        let four_by_four = Grid::from_str("#..#/..../..../#..#").unwrap();
        let parts = four_by_four.partition();
        assert_eq!(parts.len(), 4);
        assert_eq!(
            parts,
            vec![
                Grid::from_str("#./..").unwrap(),
                Grid::from_str(".#/..").unwrap(),
                Grid::from_str("../#.").unwrap(),
                Grid::from_str("../.#").unwrap(),
            ]
        );
    }

    #[test]
    fn reassemble_grid_from_parts() {
        let original = Grid::from_str("#..#/..../..../#..#").unwrap();
        let reassembled = Grid::compose(&original.partition());
        assert_eq!(original, reassembled);
    }

    #[test]
    fn flip_grid_vertically() {
        let original = Grid::from_str("##/..").unwrap();
        let flipped = original.flip_vertical();
        assert_eq!(flipped, Grid::from_str("../##").unwrap());
        assert_eq!(original, flipped.flip_vertical());
    }

    #[test]
    fn flip_grid_horizontally() {
        let original = Grid::from_str(".#/#.").unwrap();
        let flipped = original.flip_horizontal();
        assert_eq!(flipped, Grid::from_str("#./.#").unwrap());
        assert_eq!(original, flipped.flip_horizontal());
    }
}
