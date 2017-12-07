extern crate failure;
extern crate recursive_circus;

use failure::Error;
use std::process::exit;
use std::fs::File;
use std::io::{BufRead, BufReader};
use recursive_circus::{find_root, Program};

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let input = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(input);
    let programs = reader
        .lines()
        .map(|line| {
            line.map_err(failure::Error::from)
                .and_then(|l| l.parse::<Program>())
        })
        .collect::<Result<Vec<Program>, failure::Error>>()?;

    let program_at_bottom = find_root(&programs);

    println!("The program at the bottom is {}", program_at_bottom.name);

    Ok(())
}
