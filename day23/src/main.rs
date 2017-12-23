extern crate conflagration;
use conflagration::{Processor, Instruction};
use conflagration::errors::Error;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let instructions = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, Error>>()?;
    
    let mut processor = Processor::new();
    processor.execute(&instructions);

    println!("The multiply instruction was invoked {} times.", processor.multiplication_count());

    Ok(())
}