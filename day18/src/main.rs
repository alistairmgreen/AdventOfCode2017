extern crate duet;
use duet::{Instruction, Error, play, perform_duet};
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
        .map(|i| i.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, Error>>()?;
    
    let sound = play(&instructions);
    match sound {
        Some(s) => println!("Frequency of first sound played is {} Hz.", s),
        None => println!("No sound is played.")
    };

    perform_duet(&instructions);

    Ok(())
}
