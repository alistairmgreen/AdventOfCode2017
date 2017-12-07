extern crate stopwatch;

use std::num::ParseIntError;
use stopwatch::Stopwatch;

fn main() {
    let puzzle_input = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .unwrap();

    let mut stopwatch = Stopwatch::start_new();
    let part1 = maze_part_1(&mut puzzle_input.clone());
    let part2 = maze_part_2(&mut puzzle_input.clone());
    stopwatch.stop();

    println!("For part 1 rules, it takes {} jumps to escape the maze.", part1);

    println!("For part 2 rules, it takes {} jumps to escape the maze.", part2);

    println!("Elapsed time: {}", stopwatch);
}

fn maze_part_1(offsets: &mut [i32]) -> usize {
    let mut index: usize;
    let mut next_index: i32 = 0;
    let length = offsets.len() as i32;
    let mut count: usize = 0;

    while next_index < length && next_index >= 0 {
        index = next_index as usize;
        next_index = index as i32 + offsets[index];

        offsets[index] += 1;
        count += 1;
    }

    count
}

fn maze_part_2(offsets: &mut [i32]) -> usize {
    let mut index: usize;
    let mut next_index: i32 = 0;
    let length = offsets.len() as i32;
    let mut count: usize = 0;

    while next_index < length && next_index >= 0 {
        index = next_index as usize;
        next_index = index as i32 + offsets[index];

        offsets[index] += if offsets[index] >= 3 { -1 } else { 1 };
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<i32> {
        vec![0, 3, 0, 1, -3]
    }

    #[test]
    fn part1_example_requires_5_steps() {
        assert_eq!(maze_part_1(&mut example_input()), 5);
    }

    #[test]
    fn part2_example_requires_10_steps() {
        assert_eq!(maze_part_2(&mut example_input()), 10);
    }
}
