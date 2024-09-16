use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // length of vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // normalize to unit length
    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3::new(self.x / mag, self.y / mag, self.z / mag)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul for Vector3 {
    type Output = f32;

    fn mul(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
