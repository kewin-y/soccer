use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output{
        Vec3 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ]
        }
    }
}
