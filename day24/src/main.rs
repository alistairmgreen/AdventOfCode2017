extern crate electromagnetic_moat;
use electromagnetic_moat::*;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), ParseComponentError> {
    let components = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Component>, _>>()?;

    let strongest_bridge = build(&Bridge::new(), &components);

    println!(
        "The strongest bridge that can be built has strength {}.",
        strongest_bridge.strength()
    );

    Ok(())
}
