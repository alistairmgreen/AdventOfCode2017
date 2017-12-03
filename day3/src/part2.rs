use std::collections::HashMap;
use super::*;

pub struct Grid {
    occupied_squares: HashMap<Displacement, usize>,
    current_position: Displacement,
    direction: Direction,
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid = Grid {
            occupied_squares: HashMap::new(),
            current_position: Displacement::new(),
            direction: Direction::East,
        };

        grid.occupied_squares
            .insert(grid.current_position.clone(), 1);

        grid
    }
}

impl Iterator for Grid {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_position = self.current_position.step(&self.direction);

        let value = self.current_position
            .neighbours()
            .iter()
            .map(|position| {
                self.occupied_squares.get(&position).unwrap_or(&0)
            })
            .sum();

        self.occupied_squares
            .insert(self.current_position.clone(), value);

        let left_direction = self.direction.turn_left();
        let square_to_my_left = self.current_position.step(&left_direction);
        if !self.occupied_squares.contains_key(&square_to_my_left) {
            self.direction = left_direction;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_yields_correct_values() {
        /*
        Grid should look like this:
        147  142  133  122   59
        304    5    4    2   57
        330   10    1    1   54
        351   11   23   25   26
        362  747  806--->   ...

        Values spiral anticlockwise from the '1' at the centre.
        The central '1' is not included.
        */
        let values: Vec<usize> = Grid::new().take(22).collect();
        assert_eq!(
            values,
            vec![
                1,
                2,
                4,
                5,
                10,
                11,
                23,
                25,
                26,
                54,
                57,
                59,
                122,
                133,
                142,
                147,
                304,
                330,
                351,
                362,
                747,
                806,
            ]
        );
    }
}
