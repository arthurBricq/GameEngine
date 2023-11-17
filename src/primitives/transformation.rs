use crate::primitives::matrix3::Matrix3;
use crate::primitives::vector::Vector3;

/// Represent an homogenous transformation of the 3D space to the 3D space
#[derive(Debug)]
pub struct Transform {
    translation: Vector3,
    // how to represent the rotation ?
    rotation: Matrix3,
}

impl Transform {
    pub fn new(t: Vector3, R: Matrix3) -> Self {
        Self { translation: t, rotation: R }
    }

    pub fn apply(&self, vec: Vector3) -> Vector3 {
        self.rotation.clone() * (vec + self.translation)
    }
}