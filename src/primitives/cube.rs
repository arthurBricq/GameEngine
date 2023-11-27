use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::object::Object;

use crate::primitives::position::Pose;
use crate::primitives::vector::Vector3;


/// A cube in 3D coordinates.
/// The cube is defined by its faces. This is not the most lightweight representation of a cube,
/// but it seems to fit the purposes better than using 8 points.
pub struct Cube3 {
    faces: [CubicFace3; 6],
}

impl Cube3 {
    /// Construct a cube from a bottom face with an extrusion above, strictly on the z-direction
    pub fn from_face(bottom: CubicFace3, h: f32) -> Self {
        // Construct the 4 points of the upper face
        let points = bottom.points();

        // It should be noted that the following lines perform a lot of computation
        // each + creates 2 copies. Maybe we want to do something lighter.
        let extrusion_vec = Vector3::new(0.0, 0.0, h);
        let p0 = points[0] + extrusion_vec;
        let p1 = points[1] + extrusion_vec;
        let p2 = points[2] + extrusion_vec;
        let p3 = points[3] + extrusion_vec;

        // Construct the missing faces
        let n = bottom.normal();
        let texture = bottom.texture();
        let c = bottom.color();
        let top = CubicFace3::new([p0, p1, p2, p3], n.opposite(), Color::black());
        let f01 = CubicFace3::new([p0, p1, points[1], points[0]], p1 - p2, c.randomize_dimension(3));
        let f12 = CubicFace3::new([p1, p2, points[2], points[1]], p1 - p0, c.randomize_dimension(3));
        let f23 = CubicFace3::new([p2, p3, points[3], points[2]], p2 - p1, c.randomize_dimension(3));
        let f30 = CubicFace3::new([p3, p0, points[0], points[3]], p0 - p1, c.randomize_dimension(3));

        Self {
            faces: [bottom, top, f01, f12, f23, f30],
        }
    }

}

impl Object for Cube3 {
    /// Criteria for a face to be seen:
    /// * the dot product between the camera's orientation and the face's normal
    ///   is negative.
    /// * the dot product between the face's normal and the vector going to the camera is
    ///   also negative
    fn get_visible_faces(&self, camera: &Camera) -> Vec<&CubicFace3> {
        let mut to_return = Vec::new();
        for face in &self.faces {
            if face.is_visible_from(camera) {
                to_return.push(face);
            }
        }
        to_return
    }

    /// Rotate the rectangle by a provided angle
   fn rotate(&mut self, by: f32) {
        for face in &mut self.faces {
            face.rotate(by);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cube::Cube3;
    use crate::primitives::cubic_face3::CubicFace3;
    use crate::primitives::object::Object;
    use crate::primitives::position::Pose;
    use crate::primitives::vector::Vector3;

    fn cam(x: f32, y: f32, theta_z: f32) -> Camera {
        Camera::new(
            Pose::new(Vector3::new(x, y, 0.0), theta_z),
            100.,
            0.0,
            0.0
        )
    }

    #[test]
    fn visible_faces() {
        // Create a cube
        let bottom_face = CubicFace3::from_line(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            false,
            Color::purple()
        );
        println!("Bottom face = {bottom_face:?}");

        let cube = Cube3::from_face(bottom_face, 2.0);
        let cube: Box<dyn Object> = Box::new(cube);

        // when looking in the wrong direction, no face should be seen
        let cam1 = cam(2.0, 0.5, 0.0);
        let faces = cube.get_visible_faces(&cam1);
        assert_eq!(0, faces.len());

        // when looking forward, only 1 face must be seen
        let cam1 = cam(2.0, 0.5, PI);
        let faces = cube.get_visible_faces(&cam1);
        println!("{faces:#?}");
        assert_eq!(1, faces.len());

        // When looking from the side, 2 faces should be seen
        let cam1 = cam(2.0, 2.0, PI + PI / 4.);
        let faces = cube.get_visible_faces(&cam1);
        println!("{faces:#?}");
        assert_eq!(2, faces.len());

        // When looking from the side, but on top, 3 faces should be seen
        let mut cam1 = cam(2.0, 2.0, PI + PI / 4.);
        cam1.translate(&Vector3::new(0., 0., 3.));
        let faces = cube.get_visible_faces(&cam1);
        println!("{faces:#?}");
        assert_eq!(3, faces.len());

        // When looking from the side, but on bottom, 3 faces should be seen
        let mut cam1 = cam(2.0, 2.0, PI + PI / 4.);
        cam1.translate(&Vector3::new(0., 0., -3.));
        let faces = cube.get_visible_faces(&cam1);
        println!("{faces:#?}");
        assert_eq!(3, faces.len());

    }
}
