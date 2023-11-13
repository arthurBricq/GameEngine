use std::ffi::c_long;
use crate::primitives::camera::Camera;
use crate::primitives::line::Line;
use crate::primitives::point::Point2;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;

/// A cubic face is an oriented square in space
pub struct CubicFace3 {
    points: [Vector3; 4],
    normal: Vector3,
}

impl CubicFace3 {

    /// Creates an horizontal cubic face by extruding a 2D line
    pub fn from_line(p1: Vector3, p2: Vector3, clockwise: bool) -> Self {
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
        }
    }

    pub fn new(points: [Vector3; 4], normal: Vector3) -> Self {
        Self { points, normal }
    }

    pub fn points(&self) -> [Vector3; 4] {
        self.points
    }

    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }
    //
    // pub fn edges(&self) -> Vec<Line<Vector3>> {
    //     let mut to_return = Vec::new();
    //     to_return.push(Line::<Vector3>::new(self.points[0], self.points[1]));
    //     to_return.push(Line::<Vector3>::new(self.points[1], self.points[2]));
    //     to_return.push(Line::<Vector3>::new(self.points[2], self.points[3]));
    //     to_return.push(Line::<Vector3>::new(self.points[3], self.points[0]));
    //     to_return
    // }

    pub fn projection(&self, camera: &Camera) -> CubicFace2 {
        todo!("Project each point of the face and create the 2D face")
    }
}

/// A cubic face is an oriented square in space
pub struct CubicFace2 {
    points: [Point2; 4],
}
