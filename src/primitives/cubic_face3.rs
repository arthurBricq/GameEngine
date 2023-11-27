use std::fmt::{Debug, Formatter};

use crate::primitives::camera::Camera;
use crate::primitives::cubic_face2::CubicFace2;
use crate::primitives::matrix3::Matrix3;
use crate::primitives::object::Object;
use crate::primitives::textures::Texture;
use crate::primitives::vector::Vector3;

/// A cubic face is an oriented square in space
pub struct CubicFace3 {
    points: [Vector3; 4],
    normal: Vector3,
    texture: Box<dyn Texture>,
}

impl Debug for CubicFace3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "points: {:?}, {:?}, {:?}, {:?} & normal = {:?}", self.points[0], self.points[1], self.points[2], self.points[3], self.normal)
    }
}

impl CubicFace3 {
    /// Creates an horizontal cubic face by extruding a 2D line
    pub fn from_line(p1: Vector3, p2: Vector3, clockwise: bool, texture: Box<dyn Texture>) -> Self {
        let v = p2 - p1;
        let rotated = if clockwise {
            v.clockwise()
        } else {
            v.anticlockwise()
        };
        let p3 = p2 + rotated;
        let p4 = p1 + rotated;
        Self {
            points: [p1, p2, p3, p4],
            normal: Vector3::new(0.0, 0.0, -1.0),
            texture,
        }
    }

    /// Creates a face pointing in the x direction
    pub fn create_simple_face(x: f32, y: f32, z: f32, w: f32, h: f32, texture: Box<dyn Texture>) -> Self {
        CubicFace3::new([
                            Vector3::new(x, y, z),
                            Vector3::new(x, y + w, z),
                            Vector3::new(x, y + w, z - h),
                            Vector3::new(x, y, z - h),
                        ],
                        Vector3::new(-1., 0., -0.),
                        texture
        )
    }

    pub fn new(points: [Vector3; 4], normal: Vector3, texture: Box<dyn Texture>) -> Self {
        Self { points, normal, texture }
    }

    pub fn points(&self) -> [Vector3; 4] {
        self.points
    }

    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }



    pub fn projection(&self, camera: &Camera) -> CubicFace2 {
        let points2 = self.points.map(|p| camera.project(p));
        CubicFace2::new(points2, self)
    }

    pub fn center(&self) -> Vector3 {
        // TODO for efficiency, this could be computed upon creation
        (self.points[0] + self.points[1] + self.points[2] + self.points[3]) / 4.
    }

    pub fn rotate(&mut self, by: f32) {
        let mat = Matrix3::z_rotation(by);
        // rotate each point of the face
        for i in 0..4 {
            // TODO is there a way to use a reference of mat here ?
            self.points[i] = mat.clone() * self.points[i];
        }
        // The normal vector also has to be rotated
        self.normal = mat.clone() * self.normal;
    }

    pub fn is_visible_from(&self, camera: &Camera) -> bool {
        let dot1 = self.normal().dot(&camera.position().orientation());
        let cam_to_center = self.center() - *camera.position().position();
        let dot2 = self.normal().dot(&cam_to_center);
        if dot1 <= 0.0 {
            if dot2 < 0.0 {
                return true;
            }
        }
        return false;
    }
    pub fn texture(&self) -> &Box<dyn Texture> {
        &self.texture
    }
}

impl Object for CubicFace3 {
    fn get_visible_faces(&self, camera: &Camera) -> Vec<&CubicFace3> {
        if self.is_visible_from(camera) {
            vec![self]
        } else {
            // TODO maybe it's better to use an option ?
            Vec::new()
        }
    }

    fn rotate(&mut self, by: f32) {
        self.rotate(by);
    }
}



