use crate::primitives::cubic_face::CubicFace3;
use crate::primitives::line::Line;
use crate::primitives::point::Point3;
use crate::primitives::position::Position;
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
        let extrusion_vec = Vector3::new(0.0, 0.0, h);
        let p0 = points[0].add(&extrusion_vec);
        let p1 = points[1].add(&extrusion_vec);
        let p2 = points[2].add(&extrusion_vec);
        let p3 = points[3].add(&extrusion_vec);

        // Construct the missing faces
        let n = bottom.normal();
        let top = CubicFace3::new([p0, p1, p2, p3], n.opposite());
        let f01 = CubicFace3::new([p0, p1, points[1], points[0]], p1 - p2);
        let f12 = CubicFace3::new([p1, p2, points[2], points[1]], p1 - p0);
        let f23 = CubicFace3::new([p2, p3, points[3], points[2]], p2 - p1);
        let f30 = CubicFace3::new([p3, p0, points[0], points[3]], p0 - p1);

        Self {
            faces: [bottom, top, f01, f12, f23, f30],
        }
    }

    pub fn get_visible_faces(&self, from: &Position) -> Vec<&CubicFace3> {
        let mut to_return = Vec::new();
        for face in &self.faces {
            // Compute the dot product between the normal of the face and the orientation
            // The face can be seen only if the dot product is negative
            let n = face.normal().dot(from.orientation());
            if n < 0.0 {
                to_return.push(face);
            }
            // TODO the position can be used to do a better decision
        }
        to_return
    }
}
