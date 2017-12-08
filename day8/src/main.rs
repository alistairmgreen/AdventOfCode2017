extern crate registers;
extern crate failure;
use registers::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), failure::Error> {
    let input = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(input);
    let mut registers = Registers::new();

    for line in reader.lines() {
        let line = line?;
        let statement = line.parse::<Statement>()?;
        registers.execute(&statement);
    }

    println!("The largest value in any register is {}.", registers.largest_value());
    Ok(())
}