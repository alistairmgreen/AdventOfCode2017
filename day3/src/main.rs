extern crate spiral_memory;
use spiral_memory::PUZZLE_INPUT;
use spiral_memory::part1::*;
use spiral_memory::part2::Grid;

fn main() {
    println!("Part 1:");
    println!("Data from square {} requires {} steps to reach the centre.", PUZZLE_INPUT, steps_required(PUZZLE_INPUT));

    println!("Part 2:");
    let mut grid = Grid::new();
    let solution = grid.find(|&n| n > PUZZLE_INPUT).expect("Did not find a solution");
    println!("The first number greater than {} is {}.", PUZZLE_INPUT, solution);
}