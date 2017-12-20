use std::ops::{Add, AddAssign};

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
