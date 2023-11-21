use std::fmt::{Debug, Formatter};
use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::matrix3::Matrix3;
use crate::primitives::point::Point2;
use crate::primitives::vector::Vector3;

/// A cubic face is an oriented square in space.
///
/// A 2D face can hold a reference to its referring 3D face.
pub struct CubicFace2<'a> {
    points: [Point2; 4],
    color: Color,
    face3: Option<&'a CubicFace3>,
}

impl<'a> Debug for CubicFace2<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "points: {:?}, {:?}, {:?}, {:?} ", self.points[0], self.points[1], self.points[2], self.points[3])
    }
}

impl<'a> CubicFace2<'a> {
    pub fn new(points: [Point2; 4], color: Color, face: &'a CubicFace3) -> Self {
        Self { points, color, face3: Some(face) }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

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
        let c1 = is_left_of_link(&self.points, 0, 1, point);
        let c2 = is_left_of_link(&self.points, 1, 2, point);
        let c3 = is_left_of_link(&self.points, 2, 3, point);
        let c4 = is_left_of_link(&self.points, 3, 0, point);

        // Returns true if all conditions are equals
        return (c1 == c2) && (c1 == c3) && (c1 == c4);
    }

    /// Returns the raytraciing distance between the face and a ray defined as the pixels
    /// of the camera's screen.
    pub fn raytracing_distance(&self, u: i16, v: i16, camera: &Camera) -> Option<f32> {
        // Notation (*) means to be determined
        // C     = camera location
        // v     = ray's direction
        // P     = One corner of the 3D face
        // a & b = vectors from P to the adjacent corners of the face
        //
        // Equation to solve
        // C + t * v = P + alpha * a + beta * b
        // where t, alpha and beta are real numbers

        if let Some(face) = self.face3 {
            let v = camera.ray_direction(u, v);
            let points = face.points();
            let c = *camera.position().position();
            let p = points[0];
            let a = points[1] - p;
            let b = points[3] - p;
            let A = Matrix3::new(a.x(), b.x(), -v.x(),
                                 a.y(), b.y(), -v.y(),
                                 a.z(), b.z(), -v.z(),
            );
            let rhs = c - p;
            // Solve the system
            if let Some(solution) = A.linear_solve(rhs) {
                let alpha = solution.x();
                let beta = solution.y();
                let t = solution.z();
                if t >= 0. && alpha >= 0. && alpha <= 1. && beta >= 0. && beta <= 1. {
                    // This means the intersection is on the plane
                    return Some(t * v.norm());
                }
            }
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cubic_face2::CubicFace2;
    use crate::primitives::cubic_face3::CubicFace3;
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
            color: Color::purple(),
            face3: None,
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

    #[test]
    fn contains2() {
        let face2 = CubicFace2 {
            points: [
                Point2::new(160., 20.),
                Point2::new(160., 53.3),
                Point2::new(193.3, 53.3),
                Point2::new(210., 20.),
            ],
            color: Color::purple(),
            face3: None,
        };
        assert!(face2.contains(&Point2::new(161., 21.)));
    }

    #[test]
    /// Test that the raytracing algorithm works well
    fn raytracing_distance() {
        // Create a camera
        let camera = Camera::new(
            Position::new(Vector3::new(-2.0, 0., 0.), 0.0),
            100.0, 100., 100.
        );

        // Create a cubic2 face facing the camera
        let x: f32 = 0.;
        let y: f32 = -2.;
        let z: f32 = -2.;
        let face = CubicFace3::new([
                                     Vector3::new(x, y, z),
                                     Vector3::new(x, y+4., z),
                                     Vector3::new(x, y+4., z+4.),
                                     Vector3::new(x, y, z+4.),
                                 ],
                                 Vector3::new(-1., 0., 0.),
                                 Color::yellow(),
        );

        // Now let's get serious
        let projection = face.projection(&camera);
        println!("Projection = {projection:?}");

        let d1 = projection.raytracing_distance(100, 100, &camera);
        assert_eq!(d1, Some(2.));

        let d2 = projection.raytracing_distance(110, 100, &camera);
        let d3 = projection.raytracing_distance(90, 100, &camera);
        assert!(d2.unwrap() > d1.unwrap());
        assert!(d3.unwrap() > d1.unwrap());
        assert_eq!(d2, d3);

        let d4 = projection.raytracing_distance(100, 110, &camera);
        let d5 = projection.raytracing_distance(100, 90, &camera);
        assert!(d4.unwrap() > d1.unwrap());
        assert!(d5.unwrap() > d1.unwrap());
        assert_eq!(d4, d5);
        //
        // println!("{d1:#?}");
        // println!("{:#?}",  projection.raytracing_distance(110, 100, &camera));
        // println!("{:#?}",  projection.raytracing_distance(90, 100, &camera));
        // println!("{:#?}",  projection.raytracing_distance(100, 110, &camera));
        // println!("{:#?}",  projection.raytracing_distance(100, 90, &camera));
    }
}
