use std::ops::Mul;
use crate::primitives::vector::Vector3;

#[derive(Clone, Debug)]
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

    /// Create a rotation matrix around the z-axis of the plane
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

    /// Create a rotation matrix around any axis of an angle theta
    ///
    /// https://en.wikipedia.org/wiki/Transformation_matrix#Rotation_2
    pub fn rotation_around(theta: f32, mut axis: Vector3) -> Self {
        axis.normalize();
        let x = axis.x();
        let y = axis.y();
        let z = axis.z();
        let c = f32::cos(theta);
        let s = f32::sin(theta);
        Self {
            a11: x * x * (1. - c) + c,
            a21: y * x * (1. - c) - z * s,
            a31: z * x * (1. - c) + y * s,
            a12: x * y * (1. - c) + z * s,
            a22: y * y * (1. - x) + c,
            a32: z * y * (1. - c) - x * s,
            a13: x * z * (1. - c) - y * s,
            a23: y * z * (1. - c) + x * s,
            a33: z * z * (1. - c) + c,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::matrix3::Matrix3;
    use crate::primitives::vector::Vector3;

    const EPSILON: f32 = 0.0001;

    fn assert_near(left: Vector3, right: Vector3) {
        println!("{left:?} vs {right:?}");
        assert!(f32::abs(left.x() - right.x()) < EPSILON);
        assert!(f32::abs(left.y() - right.y()) < EPSILON);
        assert!(f32::abs(left.z() - right.z()) < EPSILON);
    }

    #[test]
    fn rotation_around_axis() {
        let pi = std::f64::consts::PI as f32;
        let vx = Vector3::new(1.0, 0.0, 0.0);
        let vy = Vector3::new(0.0, 1.0, 0.0);
        let vz = Vector3::new(0.0, 0.0, 1.0);

        // Check that basic rotations works as expected
        assert_near(Matrix3::rotation_around(pi, vx) * vx, vx);
        assert_near(Matrix3::rotation_around(pi, vx) * vy, vy.opposite());
        assert_near(Matrix3::rotation_around(pi, vx) * vz, vz.opposite());

        assert_near(Matrix3::rotation_around(pi / 2.0, vz) * vx, vy.opposite());
        assert_near(Matrix3::rotation_around(pi / 2.0, vz) * vy, vx);
    }
}
