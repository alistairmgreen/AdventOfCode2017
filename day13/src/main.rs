extern crate packet_scanners;
extern crate failure;

use packet_scanners::Firewall;
use failure::Error;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let firewall = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Firewall, Error>>()?;

    let severity = firewall.severity();

    println!("The severity of the firewall is {}.", severity);

    Ok(())
}
