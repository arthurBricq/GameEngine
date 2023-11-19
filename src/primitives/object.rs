use crate::primitives::camera::Camera;
use crate::primitives::cubic_face3::CubicFace3;

/// An object is a 3D element which can be part of the world
pub trait Object {
    fn get_visible_faces(&self, camera: &Camera) -> Vec<&CubicFace3>;
    fn rotate(&mut self, by: f32);
}