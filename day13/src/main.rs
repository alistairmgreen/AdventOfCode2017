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

    let delay = (0..100000000).find(|&delay| !firewall.caught_at_time_delay(delay));
    match delay {
        Some(t) =>  println!("The minimum time delay to escape detection is {} ps.", t),
        None => println!("You cannot escape detection in any sensible time."),
    }        

    Ok(())
}
