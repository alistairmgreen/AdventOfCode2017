extern crate fractal_art;
extern crate failure;
use failure::Error;
use fractal_art::{read_rules, enhance, Grid};
use std::process::exit;
use std::collections::HashMap;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let rules = get_rules()?;
    let mut grid = Grid::starting_pattern();

    for n in 1..6 {
        grid = enhance(&grid, &rules);
        println!("Iteration {}:\n{}\n", n, grid);
    }

    println!("After 5 iterations, {} pixels are on.", grid.count_pixels_on());

    for _ in 6..19 {
        grid = enhance(&grid, &rules);
    }

    println!("After 18 iterations, {} pixels are on.", grid.count_pixels_on());    

    Ok(())
}

fn get_rules() -> Result<HashMap<Grid, Grid>, Error> {
    let input = include_str!("puzzle_input.txt");
    read_rules(input.lines())
}