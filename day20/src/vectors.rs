use std::ops::{Add, AddAssign};
use std::str::FromStr;
use std::num::ParseIntError;
use failure::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vector3D {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Vector3D {
    pub fn new(x: i64, y: i64, z: i64) -> Vector3D {
        Vector3D {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl FromStr for Vector3D {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;
        
        if parts.len() < 3 {
            bail!("{} is not a valid 3D vector.", s);
        }

        Ok(Vector3D::new(parts[0], parts[1], parts[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vector() {
        let v: Vector3D = "-833,-499,-1391".parse().unwrap();
        assert_eq!(v, Vector3D::new(-833, -499, -1391));
    }
}