extern crate spiral_memory;
use spiral_memory::PUZZLE_INPUT;
use spiral_memory::part1::*;

fn main() {
    println!("Data from square {} requires {} steps", PUZZLE_INPUT, steps_required(PUZZLE_INPUT));
}