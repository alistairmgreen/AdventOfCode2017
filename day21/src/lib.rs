#[macro_use]
extern crate failure;
use std::str::FromStr;
use failure::Error;
use std::fmt;
use std::collections::HashMap;

fn isqrt(n: usize) -> usize {
    let root = (n as f32).sqrt();
    root.floor() as usize
}

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

impl Grid {
    pub fn starting_pattern() -> Grid {
        Grid {
            elements: vec![
                Pixel::Off,
                Pixel::On,
                Pixel::Off,
                Pixel::Off,
                Pixel::Off,
                Pixel::On,
                Pixel::On,
                Pixel::On,
                Pixel::On,
            ],
            size: 3,
        }
    }

    pub fn count_pixels_on(&self) -> usize {
        self.elements
            .iter()
            .filter(|&&pixel| pixel == Pixel::On)
            .count()
    }

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
        let pieces_per_side = isqrt(pieces.len());
        let size = piece_size * pieces_per_side;
        let mut elements: Vec<Pixel> = Vec::with_capacity(size * size);

        for row_pieces in pieces.chunks(pieces_per_side) {
            for row in 0..piece_size {
                for piece in row_pieces {
                    elements.extend_from_slice(piece.row(row));
                }
            }
        }

        Grid {
            elements: elements,
            size: size,
        }
    }

    fn flip_vertical(&self) -> Grid {
        let mut elements: Vec<Pixel> = Vec::with_capacity(self.elements.len());
        for row in (0..self.size).rev() {
            elements.extend_from_slice(self.row(row));
        }

        Grid {
            elements: elements,
            size: self.size,
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
            size: self.size,
        }
    }

    fn rotate(&self) -> Grid {
        let mut elements: Vec<Pixel> = Vec::with_capacity(self.elements.len());

        for column in 0..self.size {
            for row in (0..self.size).rev() {
                elements.push(self.elements[row * self.size + column]);
            }
        }

        Grid {
            elements: elements,
            size: self.size,
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

pub fn read_rules<'a, T: Iterator<Item = &'a str>>(rules: T) -> Result<HashMap<Grid, Grid>, Error> {
    let mut table = HashMap::new();

    for rule_string in rules {
        let parts: Vec<&str> = rule_string.split("=>").collect();
        if parts.len() != 2 {
            bail!("Cannot parse rule: '{}'", rule_string);
        }

        let small: Grid = parts[0].trim().parse()?;
        let large: Grid = parts[1].trim().parse()?;

        add_rotations(&small, &large, &mut table);

        let flip_ud = small.flip_vertical();
        add_rotations(&flip_ud, &large, &mut table);
        table.insert(flip_ud, large.clone());

        let flip_lr = small.flip_horizontal();
        add_rotations(&flip_lr, &large, &mut table);

        table.insert(small, large);
    }

    Ok(table)
}

fn add_rotations(pattern: &Grid, enhanced: &Grid, rules: &mut HashMap<Grid, Grid>) {
    let rotated90 = pattern.rotate();
    let rotated180 = rotated90.rotate();
    let rotated270 = rotated180.rotate();
    rules.insert(rotated90, enhanced.clone());
    rules.insert(rotated180, enhanced.clone());
    rules.insert(rotated270, enhanced.clone());
}

pub fn enhance<S>(grid: &Grid, rules: &HashMap<Grid, Grid, S>) -> Grid
where
    S: ::std::hash::BuildHasher,
{
    let enhanced_parts: Vec<Grid> = grid.partition()
        .iter()
        .map(|piece| {
            rules
                .get(piece)
                .expect(&format!("Could not find enhancement rule for:\n{}", piece))
        })
        .cloned()
        .collect();

    Grid::compose(&enhanced_parts)
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
        assert_eq!(
            grid.row(1),
            &[Pixel::Off, Pixel::Off, Pixel::Off, Pixel::Off]
        );
        assert_eq!(
            grid.row(2),
            &[Pixel::Off, Pixel::Off, Pixel::Off, Pixel::Off]
        );
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
    fn compose_single_grid() {
        let original = vec![Grid::from_str("#./..").unwrap()];
        let composed = Grid::compose(&original);
        assert_eq!(composed, original[0]);
    }

    #[test]
    fn compose_four_3x3_grids() {
        let unit = Grid::from_str("##./#../...").unwrap();
        let parts = vec![unit; 4];
        let composed = Grid::compose(&parts);
        let expected = Grid::from_str("##.##./#..#../....../##.##./#..#../......").unwrap();

        assert_eq!(composed, expected);
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

    #[test]
    fn rotate_grid() {
        let original = Grid::from_str(".#./..#/###").unwrap();
        let rotated = original.rotate();
        assert_eq!(rotated, Grid::from_str("#../#.#/##.").unwrap());
    }

    #[test]
    fn part_1_example() {
        let input_rules = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let rules = read_rules(input_rules.lines()).unwrap();
        let grid = Grid::starting_pattern();

        let step1 = enhance(&grid, &rules);
        assert_eq!(
            step1,
            Grid::from_str("#..#/..../..../#..#").unwrap(),
            "Step 1 is incorrect"
        );

        let step2 = enhance(&step1, &rules);

        assert_eq!(
            step2,
            Grid::from_str("##.##./#..#../....../##.##./#..#../......").unwrap(),
            "Step 2 is incorrect"
        );

        assert_eq!(step2.count_pixels_on(), 12);
    }
}
