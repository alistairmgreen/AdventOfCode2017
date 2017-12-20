extern crate particle_swarm;
extern crate failure;
use failure::Error;

use particle_swarm::*;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let particles = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse::<Particle>())
        .collect::<Result<Vec<Particle>, Error>>()?;
    
    let closest = simulate(particles);

    println!("In the long run, the closest particle is number {}.", closest);

    Ok(())
}