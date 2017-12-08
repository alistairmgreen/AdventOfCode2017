extern crate failure;
extern crate recursive_circus;

use failure::Error;
use std::process::exit;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use recursive_circus::*;

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

    println!("Programs that are imbalanced, but have balanced children:");

    for (name, program) in programs.iter().filter(|&(name, program)| {
        !is_balanced(name, &programs)
            && program
                .children
                .iter()
                .all(|child| is_balanced(child, &programs))
    }) {
        println!("{}", name);
        println!("Its children are:");
        for child in &program.children {
            println!("{} (weight {}, total weight {})", child, programs[child].weight, total_weight(child, &programs));
        }
    }

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
