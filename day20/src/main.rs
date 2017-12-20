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
    
    let closest = simulate(particles.clone());

    println!("Part 1: In the long run, the closest particle is number {}.", closest);

    let remaining = simulate_with_collisions(particles);

    println!("Part 2: {} particles are left after all collisions have been resolved.", remaining);

    Ok(())
}