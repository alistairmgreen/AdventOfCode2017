#[macro_use]
extern crate failure;

pub mod vectors;

use std::str::FromStr;
use std::collections::HashMap;
use vectors::Vector3D;
use failure::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Particle {
    position: Vector3D,
    velocity: Vector3D,
    acceleration: Vector3D,
}

impl Particle {
    pub fn new(position: Vector3D, velocity: Vector3D, acceleration: Vector3D) -> Particle {
        Particle {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }

    pub fn distance_from_origin(&self) -> i64 {
        self.position.manhattan_distance()
    }

    pub fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace()
            .map(|s| {
                s.chars()
                    .skip(3)
                    .take_while(|&c| c != '>')
                    .collect::<String>()
                    .parse::<Vector3D>()
            })
            .collect::<Result<Vec<Vector3D>, Error>>()?;

        if parts.len() < 3 {
            bail!("{} is not a valid particle.", s);
        }

        Ok(Particle::new(parts[0], parts[1], parts[2]))
    }
}

fn find_closest(particles: &[Particle]) -> usize {
    let (index, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| p.distance_from_origin())
        .unwrap();

    index
}

pub fn simulate(mut particles: Vec<Particle>) -> usize {
    let mut closest: usize = find_closest(&particles);
    let mut previous_closest: usize;
    let mut times_with_same_result: usize = 0;

    while times_with_same_result < 1000 {
        for particle in &mut particles {
            particle.tick();
        }

        previous_closest = closest;
        closest = find_closest(&particles);

        times_with_same_result = if closest == previous_closest {
            times_with_same_result + 1
        } else {
            0
        };
    }

    closest
}

pub fn simulate_with_collisions(mut particles: Vec<Particle>) -> usize {
    let mut particle_count = particles.len();
    let mut previous_count;
    let mut times_with_same_result: usize = 0;

    while times_with_same_result < 1000 {
        previous_count = particle_count;

        for particle in &mut particles {
            particle.tick();
        }

        remove_colliding(&mut particles);
        particle_count = particles.len();

        times_with_same_result = if particle_count == previous_count {
            times_with_same_result + 1
        } else {
            0
        };
    }

    particle_count
}

fn remove_colliding(particles: &mut Vec<Particle>) {
    let mut number_at_position: HashMap<Vector3D, usize> = HashMap::with_capacity(particles.len());

    for particle in particles.iter() {
        *number_at_position.entry(particle.position).or_insert(0) += 1;
    }

    particles.retain(|p| number_at_position[&p.position] == 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_particles() -> Vec<Particle> {
        vec![
            Particle {
                position: Vector3D::new(3, 0, 0),
                velocity: Vector3D::new(2, 0, 0),
                acceleration: Vector3D::new(-1, 0, 0),
            },
            Particle {
                position: Vector3D::new(4, 0, 0),
                velocity: Vector3D::new(0, 0, 0),
                acceleration: Vector3D::new(-2, 0, 0),
            },
        ]
    }

    #[test]
    fn test_find_closest_particle() {
        let particles = example_particles();

        assert_eq!(find_closest(&particles), 0);
    }

    #[test]
    fn simulate_finds_closest_particle_in_long_term() {
        let particles = example_particles();
        assert_eq!(simulate(particles), 0);
    }

    #[test]
    fn test_parse_particle() {
        let p: Particle = "p=<-833,-499,-1391>, v=<84,17,61>, a=<-4,1,1>"
            .parse()
            .unwrap();
        assert_eq!(p.position, Vector3D::new(-833, -499, -1391));
        assert_eq!(p.velocity, Vector3D::new(84, 17, 61));
        assert_eq!(p.acceleration, Vector3D::new(-4, 1, 1));
    }

    #[test]
    fn test_remove_colliding_particles() {
        let mut particles = vec![
            Particle {
                position: Vector3D::new(0, 0, 0),
                velocity: Vector3D::new(3, 0, 0),
                acceleration: Vector3D::new(0, 0, 0),
            },
            Particle {
                position: Vector3D::new(0, 0, 0),
                velocity: Vector3D::new(2, 0, 0),
                acceleration: Vector3D::new(0, 0, 0),
            },
            Particle {
                position: Vector3D::new(0, 0, 0),
                velocity: Vector3D::new(1, 0, 0),
                acceleration: Vector3D::new(0, 0, 0),
            },
            Particle {
                position: Vector3D::new(1, 0, 0),
                velocity: Vector3D::new(-1, 0, 0),
                acceleration: Vector3D::new(0, 0, 0),
            },
        ];

        remove_colliding(&mut particles);

        assert_eq!(
            particles,
            vec![
                Particle {
                    position: Vector3D::new(1, 0, 0),
                    velocity: Vector3D::new(-1, 0, 0),
                    acceleration: Vector3D::new(0, 0, 0),
                },
            ]
        );
    }
}
