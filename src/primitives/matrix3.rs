use std::ops::Mul;
use crate::primitives::vector::Vector3;

#[derive(Clone)]
pub struct Matrix3 {
    a11: f32,
    a12: f32,
    a13: f32,
    a21: f32,
    a22: f32,
    a23: f32,
    a31: f32,
    a32: f32,
    a33: f32,
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self.a11 * rhs.x() + self.a12 * rhs.y() + self.a13 * rhs.z(),
            self.a21 * rhs.x() + self.a22 * rhs.y() + self.a23 * rhs.z(),
            self.a31 * rhs.x() + self.a32 * rhs.y() + self.a33 * rhs.z(),
        )
    }
}

impl Matrix3 {
    pub fn new(a11: f32, a12: f32, a13: f32, a21: f32, a22: f32, a23: f32, a31: f32, a32: f32, a33: f32) -> Self {
        Self { a11, a12, a13, a21, a22, a23, a31, a32, a33 }
    }

    /// Create a matrix from its 3 rows
    pub fn from_columns(c1: [f32; 3], c2: [f32; 3], c3: [f32; 3]) -> Self {
        Self {
            a11: c1[0],
            a21: c1[1],
            a31: c1[2],
            a12: c2[0],
            a22: c2[1],
            a32: c2[2],
            a13: c3[0],
            a23: c3[1],
            a33: c3[2],
        }
    }

    pub fn identity() -> Self {
        Self {
            a11: 1.0,
            a21: 0.0,
            a31: 0.0,
            a12: 0.0,
            a22: 1.0,
            a32: 0.0,
            a13: 0.0,
            a23: 0.0,
            a33: 1.0,
        }
    }

    pub fn z_rotation(theta_z: f32) -> Self {
        Self {
            a11: f32::cos(theta_z),
            a21: f32::sin(theta_z),
            a31: 0.0,
            a12: -f32::sin(theta_z),
            a22: f32::cos(theta_z),
            a32: 0.0,
            a13: 0.0,
            a23: 0.0,
            a33: 1.0,
        }
    }
}