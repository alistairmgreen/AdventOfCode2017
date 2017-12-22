extern crate sporifica;
use sporifica::{InfiniteGrid, infect};

fn main() {
    let mut grid: InfiniteGrid = include_str!("puzzle_input.txt").parse().unwrap();
    let iterations = 10_000;
    let infections = infect(&mut grid, iterations);
    println!("After {} iterations, there were {} new infections.", iterations, infections);
}
