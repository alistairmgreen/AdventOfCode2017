extern crate conflagration;
use conflagration::{primes_up_to, Instruction, Processor};
use conflagration::errors::Error;
use std::process::exit;
use std::collections::HashSet;

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

    println!("Running in debug mode:");
    let mut processor = Processor::debug();
    processor.execute(&instructions);

    println!(
        "The multiply instruction was invoked {} times.",
        processor.multiplication_count()
    );
    println!("h = {}", processor.get_register('h'));

    println!("Part 2:");
    let h = part2();
    println!("h = {}", h);
    Ok(())
}

// The "assembly language" program is equivalent to:
//
// for b = 107900 to 124900 in steps of 17
//     if b is not prime
//        h = h + 1
fn part2() -> usize {
    let mut b = 107_900;
    let c = 124_900;
    let mut h = 0;
    let primes = primes_up_to(c).iter().cloned().collect::<HashSet<usize>>();

    while b <= c {
        if !primes.contains(&b) {
            h += 1;
        }

        b += 17;
    }

    h
}
