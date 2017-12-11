use std::ops::{Add, AddAssign};
use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::process::exit;

fn main() {
    let puzzle_input = include_str!("puzzle_input.txt");
    match distance_from_origin(&puzzle_input) {
        Ok(distance) => println!("Distance from origin is {}", distance),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    }
}

/*
    Note on coordinates
    -------------------

    We can think of the hexagonal grid as being a 2D slice across the
    diagonal of a 3D grid of cubes. This enables us to describe positions
    on the hexagonal grid in terms of Cartesian coordinates with axes running
    along the edges of the cubes.

    The projections of the Cartesian axes onto the plane of the hex grid point
    towards the corners of the hexagon, _not_ in the directions that we are allowed
    to step in:

     +y      -z
       \____/
       /    \
  -x__/      \___+x
      \      /
       \____/  
       /    \
      +z     -y
    
    We can move from one hex to another along the compass directions N, S, NW, NE, SW and SE.
    Writing these as row vectors in the form (x y z):
    North is (0 1 -1) because it is between +y and -z, but orthogonal to x.
    North-east is (1 0 -1) because it is between +x and -z, but orthogonal to y.
    Etc.

    For more information see https://www.redblobgames.com/grids/hexagons/
*/

#[derive(Debug, Eq, PartialEq)]
struct HexVector {
    x: i32,
    y: i32,
    z: i32
}

impl HexVector {
    pub fn origin() -> HexVector {
        HexVector { x: 0, y: 0, z: 0 }
    }

    pub fn north() -> HexVector {
        HexVector { x: 0, y: 1, z: -1 }
    }

    pub fn north_east() -> HexVector {
        HexVector { x: 1, y: 0, z: -1 }
    }

    pub fn south_east() -> HexVector {
        HexVector { x: 1, y: -1, z: 0 }
    }

    pub fn south() -> HexVector {
        HexVector { x: 0, y: -1, z: 1 }
    }

    pub fn south_west() -> HexVector {
        HexVector { x: -1, y: 0, z: 1 }
    }

    pub fn north_west() -> HexVector {
        HexVector { x: -1, y: 1, z: 0 }
    }

    pub fn magnitude(&self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) / 2
    }
}

impl Add for HexVector {
    type Output = HexVector;

    fn add(self, other: HexVector) -> HexVector {
        HexVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for HexVector {
    fn add_assign(&mut self, rhs: HexVector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl FromStr for HexVector {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(HexVector::north()),
            "ne" => Ok(HexVector::north_east()),
            "se" => Ok(HexVector::south_east()),
            "s" => Ok(HexVector::south()),
            "sw" => Ok(HexVector::south_west()),
            "nw" => Ok(HexVector::north_west()),
            _ => Err(ParseDirectionError::new(s))
        }
    }
}

#[derive(Debug)]
pub struct ParseDirectionError {
    direction: String,
}

impl ParseDirectionError {
    pub fn new(direction: &str) -> ParseDirectionError {
        ParseDirectionError {
            direction: direction.to_string()
        }
    }
}

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognised direction '{}'", self.direction)
    }
}

impl Error for ParseDirectionError {
    fn description(&self) -> &str {
        "Unrecognised direction"
    }
}

pub fn distance_from_origin(steps: &str) -> Result<i32, ParseDirectionError> {
    let mut displacement = HexVector::origin();
    for step in steps.split(',') {
        let x = HexVector::from_str(step.trim())?;
        displacement += x;
    }

    Ok(displacement.magnitude())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ne_ne_ne_is_3_steps() {
        let ne_ne_ne = HexVector::north_east() + HexVector::north_east() + HexVector::north_east();
        assert_eq!(ne_ne_ne.magnitude(), 3);
    }

    #[test]
    fn ne_ne_sw_sw_is_0_steps() {
        let ne_ne_sw_sw = HexVector::north_east()
            + HexVector::north_east()
            + HexVector::south_west()
            + HexVector::south_west();
        
        assert_eq!(ne_ne_sw_sw.magnitude(), 0);
    }

    #[test]
    fn ne_ne_s_s_is_2_steps() {
        let ne_ne_s_s = HexVector::north_east()
            + HexVector::north_east()
            + HexVector::south()
            + HexVector::south();
        
        assert_eq!(ne_ne_s_s, HexVector::south_east() + HexVector::south_east());
        assert_eq!(ne_ne_s_s.magnitude(), 2);
    }

    #[test]
    fn se_sw_se_sw_sw_is_3_steps() {
        let se_sw_se_sw_sw = 
            HexVector::south_east()
            + HexVector::south_west()
            + HexVector::south_east()
            + HexVector::south_west()
            + HexVector::south_west();
        
        assert_eq!(se_sw_se_sw_sw, HexVector::south() + HexVector::south() + HexVector::south_west());
        assert_eq!(se_sw_se_sw_sw.magnitude(), 3);
    }

    #[test]
    fn calculate_correct_distance_from_string() {
        assert_eq!(distance_from_origin("se,sw,se,sw,sw").unwrap(), 3);
    }
}