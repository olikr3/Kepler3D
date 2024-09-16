
// formal background: https://www.sjbrown.co.uk/posts/representing-rotations-in-quaternion-arithmetic/

use std::ops::{Add, Sub, Mul};
use crate::matrix::Matrix;

#[derive(Debug, Eq, PartialEq)]
pub struct Quaternion {
    /// Quaternion for 3D rotations, q = w + xi + yj + zk for w, x, y, z ∈ R
    /// Properties:
    /// i^2 = j^2 = k^2 = −1
    /// ij = k
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    /// q* = w - xi - yj - zk
    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// q_normalized = q / |q|
    pub fn normalize(&mut self) {
        let magnitude = (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        if magnitude != 0.0 {
            self.w /= magnitude;
            self.x /= magnitude;
            self.y /= magnitude;
            self.z /= magnitude;
        }
    }

    /// Converts the quaternion to a 3x3 rotation matrix
    /// This is used to apply the quaternion's rotation to vectors
    pub fn to_matrix(&self) -> Matrix {
        let mat_rows: u8 = 3;
        let mat_cols: u8 = 3;
        let mut data: Vec<f32> = vec![0.0; 9]; // 3x3 matrix in row-major order

        let qw = self.w;
        let qx = self.x;
        let qy = self.y;
        let qz = self.z;

        // Fill the matrix based on the quaternion formula for 3D rotations
        data[0] = 1.0 - 2.0 * (qy * qy + qz * qz);
        data[1] = 2.0 * (qx * qy - qz * qw);
        data[2] = 2.0 * (qx * qz + qy * qw);

        data[3] = 2.0 * (qx * qy + qz * qw);
        data[4] = 1.0 - 2.0 * (qx * qx + qz * qz);
        data[5] = 2.0 * (qy * qz - qx * qw);

        data[6] = 2.0 * (qx * qz - qy * qw);
        data[7] = 2.0 * (qy * qz + qx * qw);
        data[8] = 1.0 - 2.0 * (qx * qx + qy * qy);

        Matrix::new(mat_rows, mat_cols, data)
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Quaternion multiplication represents composition of rotations
impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}