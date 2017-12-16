extern crate permutation_promenade;
use permutation_promenade::*;
use permutation_promenade::errors::*;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), ParseDanceMoveError> {
    let input = include_str!("puzzle_input.txt");
    let mut programs = letters('a', 'p');
    for item in input.split(',') {
        let dance_move: DanceMove = item.parse()?;
        programs.perform(&dance_move);
    }

    for letter in programs {
        print!("{}", letter);
    }
    println!();

    Ok(())
}
