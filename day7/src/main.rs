extern crate failure;
extern crate recursive_circus;

use failure::Error;
use std::process::exit;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use recursive_circus::{find_root, Program};

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let programs = read_programs()?;
    let program_at_bottom = find_root(&programs);

    println!("The program at the bottom is {}", program_at_bottom.name);

    Ok(())
}

fn read_programs() -> Result<HashMap<String, Program>, failure::Error> {
    let input = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(input);
    reader
        .lines()
        .map(|line| {
            line.map_err(Error::from)
                .and_then(|l| l.parse::<Program>())
                .map(|program| (program.name.clone(), program))
        })
        .collect()
}