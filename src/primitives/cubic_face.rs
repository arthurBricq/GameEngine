use std::fmt::{Debug, Formatter};
use std::ptr::write;
use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::matrix3::Matrix3;

use crate::primitives::point::Point2;

use crate::primitives::vector::Vector3;

/// A cubic face is an oriented square in space
pub struct CubicFace3 {
    points: [Vector3; 4],
    normal: Vector3,
    color: Color
}

impl Debug for CubicFace3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "points: {:?}, {:?}, {:?}, {:?} & normal = {:?}", self.points[0], self.points[1], self.points[2], self.points[3], self.normal)
    }
}

impl CubicFace3 {
    /// Creates an horizontal cubic face by extruding a 2D line
    pub fn from_line(p1: Vector3, p2: Vector3, clockwise: bool, color: Color) -> Self {
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
            color
        }
    }

    pub fn new(points: [Vector3; 4], normal: Vector3, color: Color) -> Self {
        Self { points, normal, color }
    }

    pub fn points(&self) -> [Vector3; 4] {
        self.points
    }

    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }

    pub fn center(&self) -> Vector3 {
        // TODO for efficiency, this could be computed upon creation
        (self.points[0] + self.points[1] + self.points[2] + self.points[3]) / 4.
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn projection(&self, camera: &Camera) -> CubicFace2 {
        let points2 = self.points.map(|p| camera.project(p));
        CubicFace2 {
            points: points2,
            color: self.color.clone()
        }
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
}

/// A cubic face is an oriented square in space
pub struct CubicFace2 {
    points: [Point2; 4],
    // A face contains its colors
    color: Color
}

impl Debug for CubicFace2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "points: {:?}, {:?}, {:?}, {:?} ", self.points[0], self.points[1], self.points[2], self.points[3])
    }
}

impl CubicFace2 {
    pub fn contains(&self, point: &Point2) -> bool {
        /// Returns true if the link between the points 'i' and 'j' has the `point` to
        /// its left.
        /// The logic is done using a cross-product check
        /// https://stackoverflow.com/a/2752753/13219173
        fn is_left_of_link(points: &[Point2], i: usize, j: usize, point: &Point2) -> bool {
            let x1 = points[i].x();
            let x2 = points[j].x();
            let y1 = points[i].y();
            let y2 = points[j].y();
            let cross_product = (x2 - x1) * (point.y() - y1) - (point.x() - x1) * (y2 - y1);
            cross_product >= 0.
        }

        // The point is contained inside the face if it is to the left of all segments
        is_left_of_link(&self.points, 0, 1, point)
            && is_left_of_link(&self.points, 1, 2, point)
            && is_left_of_link(&self.points, 2, 3, point)
            && is_left_of_link(&self.points, 3, 0, point)
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cubic_face::{CubicFace2, CubicFace3};
    use crate::primitives::point::Point2;
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    #[test]
    fn contains() {
        let face2 = CubicFace2 {
            points: [
                Point2::new(0., 0.),
                Point2::new(1., 0.),
                Point2::new(1., 1.),
                Point2::new(0., 1.),
            ],
            color: Color::purple()
        };

        assert!(face2.contains(&Point2::new(0.5, 0.5)));
        assert!(face2.contains(&Point2::new(0.7, 0.6)));
        assert!(face2.contains(&Point2::new(0.1, 0.0)));
        assert!(face2.contains(&Point2::new(0.2, 0.0)));

        assert!(!face2.contains(&Point2::new(1.5, 0.5)));
        assert!(!face2.contains(&Point2::new(0.5, 1.5)));
        assert!(!face2.contains(&Point2::new(1.5, 1.5)));
        assert!(!face2.contains(&Point2::new(-1.5, 0.5)));
        assert!(!face2.contains(&Point2::new(0.5, -1.5)));
        assert!(!face2.contains(&Point2::new(-1.5, -1.5)));
    }
}


