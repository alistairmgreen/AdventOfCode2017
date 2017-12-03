use std::collections::HashSet;
use super::*;

pub fn steps_required(n: usize) -> i32 {
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