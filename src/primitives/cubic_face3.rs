use std::cmp::min;
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
    pub fn from_line(p1: Vector3, p2: Vector3, texture: Box<dyn Texture>) -> Self {
        let v = p2 - p1;
        let rotated= v.anticlockwise();
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
                        texture,
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

    pub fn texture(&self) -> &Box<dyn Texture> {
        &self.texture
    }

    pub fn projection(&self, camera: &Camera) -> CubicFace2 {
        let points2 = self.points.map(|p| camera.project(&p));
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
        let cam_to_center = self.center() - *camera.pose().position();
        let dot2 = self.normal().dot(&cam_to_center);
        if dot2 < 0.0 {
            // if the normal of the face and the vector going to the face are opposed, then
            // the face is not visible
            return false;
        }
        // The face is visible if any of the points is visible as well.
        for point in self.points {

        }
        return true;
    }

    /// Returns the closest distance from the camera to any of the line defining
    /// this polygon.
    ///
    /// See this page for the equations: https://stackoverflow.com/a/10984080/13219173
    ///
    /// This function is helpful for implementing the painter's algorithm.
    pub fn distance_to(&self, cam: &Camera) -> f32 {
        let mut d1 = distance_to_line(&self.points[0], &self.points[1], cam.pose().position());
        let mut d2 = distance_to_line(&self.points[1], &self.points[2], cam.pose().position());
        let mut d3 = distance_to_line(&self.points[2], &self.points[3], cam.pose().position());
        let mut d4 = distance_to_line(&self.points[3], &self.points[0], cam.pose().position());
        // since f64 does not implements Ord, we manually create a min of the 4 values
        if d1 <= d2 && d1 <= d3 && d1 <= d4 { d1 } else if d2 <= d1 && d2 <= d3 && d2 <= d4 { d2 } else if d3 <= d1 && d3 <= d2 && d3 <= d4 { d3 } else if d4 <= d1 && d4 <= d2 && d4 <= d3 { d4 } else { d1 }
    }
}

/// Computes the distance between the line constructed between the two provided points [p1,p2] and
/// a third point `from`.
///
/// See this page for the equations: https://stackoverflow.com/a/10984080/13219173
///
/// This function is helpful for implementing the painter's algorithm.
fn distance_to_line(p1: &Vector3, p2: &Vector3, from: &Vector3) -> f32 {
    let u = p1.line_to(from);
    let v = p1.line_to(p2);
    let r = u.dot(&v) / v.norm().powf(2.0);
    if r < 0. {
        u.norm()
    } else if r >= 0. && r < 1. {
        f32::sqrt(u.norm().powf(2.0) - (r * v.norm()).powf(2.))
    } else {
        from.line_to(p2).norm()
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


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cubic_face3::{CubicFace3, distance_to_line};
    use crate::primitives::position::Pose;
    use crate::primitives::textures::colored::ColoredTexture;
    use crate::primitives::vector::Vector3;

    #[test]
    fn visible_face_in_different_directions() {
        // Create a camera
        let mut camera = Camera::new(
            Pose::new(Vector3::new(-2.0, 0., 0.), 0.0),
            100.0, 100., 100.,
        );

        // Create a cubic2 face facing the camera
        let x: f32 = 0.;
        let y: f32 = -2.;
        let z: f32 = -2.;
        let face = CubicFace3::new([
                                       Vector3::new(x, y, z),
                                       Vector3::new(x, y + 4., z),
                                       Vector3::new(x, y + 4., z + 4.),
                                       Vector3::new(x, y, z + 4.),
                                   ],
                                   Vector3::new(-1., 0., 0.),
                                   Box::new(ColoredTexture::new(Color::dark_blue())),
        );

        // Initially the camera is looking in front
        assert!(face.is_visible_from(&camera));

        // Rotate the camera and it will look the other direction
        camera.apply_z_rot(PI);
        assert!(!face.is_visible_from(&camera));

        // Rotate it again and the face is again visible
        camera.apply_z_rot(PI);
        assert!(face.is_visible_from(&camera));

        // When performing small rotations, the face is still visible
        camera.apply_z_rot(PI / 16.);
        assert!(face.is_visible_from(&camera));
        camera.apply_z_rot(PI / 16.);
        assert!(face.is_visible_from(&camera));
        camera.apply_z_rot(-PI / 16.);
        assert!(face.is_visible_from(&camera));
        camera.apply_z_rot(-PI / 16.);
        assert!(face.is_visible_from(&camera));
        camera.apply_z_rot(-PI / 16.);
        assert!(face.is_visible_from(&camera));
        camera.apply_z_rot(-PI / 16.);
        assert!(face.is_visible_from(&camera));
    }

    fn assert_near(v1: f32, v2: f32) {
        assert!((v1 - v2).abs() < 0.0001);
    }

    #[test]
    fn test_distance_to_line() {
        let p1 = Vector3::new(0., 0., 0.);
        let p2 = Vector3::new(0., 1., 0.);

        // The edges yields no distance
        assert_near(0.0, distance_to_line(&p1, &p2, &p1));
        assert_near(0.0, distance_to_line(&p1, &p2, &p2));

        // Same for points on the line
        assert_near(0.0, distance_to_line(&p1, &p2, &Vector3::new(0.0, 0.3, 0.0)));
        assert_near(0.0, distance_to_line(&p1, &p2, &Vector3::new(0.0, 0.7, 0.0)));

        // Check points at the center on the sides
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(0.5, 0.0, 0.0)));
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(-0.5, 0.0, 0.0)));

        // And at other altitudes
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(0.5, 0.2, 0.0)));
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(-0.5, 0.8, 0.0)));

        // Bottom and top
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(0.0, 1.5, 0.0)));
        assert_near(0.5, distance_to_line(&p1, &p2, &Vector3::new(0.0, -0.5, 0.0)));

        // Just a last few checks
        assert_near(0.5 * f32::sqrt(2.), distance_to_line(&p1, &p2, &Vector3::new(0.5, 1.5, 0.0)));
        assert_near(0.5 * f32::sqrt(2.), distance_to_line(&p1, &p2, &Vector3::new(-0.5, 1.5, 0.0)));
    }
}


