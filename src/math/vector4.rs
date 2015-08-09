use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {x: x, y: y, z: z, w: w}
    }

    pub fn one() -> Vector4 {
        Vector4 {x: 1.0, y: 1.0, z: 1.0, w: 1.0}
    }

    pub fn unit_x() -> Vector4 {
        Vector4 {x: 1.0, y: 0.0, z: 0.0, w: 0.0}
    }

    pub fn unit_y() -> Vector4 {
        Vector4 {x: 0.0, y: 1.0, z: 0.0, w: 0.0}
    }

    pub fn unit_z() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 1.0, w: 0.0}
    }

    pub fn unit_w() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 0.0, w: 1.0}
    }

    pub fn zero() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w}
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w}
    }
}

impl Mul for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w}
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Vector4 {
        Vector4 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs}
    }
}

impl Div for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: Vector4) -> Vector4 {
        Vector4 {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w}
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Vector4 {
        Vector4 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs}
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Vector4 {
        Vector4 {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}
