use std::fmt::{Debug, Formatter};

use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::matrix3::Matrix3;
use crate::primitives::point::Point2;
use crate::primitives::textures::Texture;
use crate::WIDTH;

/// Contains the projected coordinates (alpha, beta) such that a point P belonging to
/// a parallelogram can be written as
///
/// P = alpha * a + beta * b + P0
///
/// where
/// * P0 = first point of the parallelogram
/// * a = vector from P0 to P1
/// * b = vector from P0 to P3
#[derive(PartialEq, Debug)]
pub struct ProjectionCoordinates {
    alpha: f32,
    beta: f32,
}

impl ProjectionCoordinates {
    pub fn new(alpha: f32, beta: f32) -> Self {
        Self { alpha, beta }
    }

    pub fn none() -> Self {
        Self {
            alpha: 0.,
            beta: 0.,
        }
    }

    pub fn to_uv(&self, norm_a: f32, norm_b: f32) -> (f32, f32) {
        (self.alpha * norm_a, self.beta * norm_b)
    }
    pub fn alpha(&self) -> f32 {
        self.alpha
    }
    pub fn beta(&self) -> f32 {
        self.beta
    }
}

/// A cubic face is an oriented square in space.
///
/// Internal properties:
/// * face: A 2D face can hold a reference to its referring 3D face.
/// * norm_a or b: the length of the side of the face. This is helpful to keep it in the class
///   to avoid.
///
/// The coordinates of the Face2 (the image referential) are defined as
///
///             u
///         ──────────►        IMAGE
///
///        ┌─────────────────────────────────┐
///    │   │                                 │
///    │   │                                 │
/// v  │   │                                 │
///    │   │                                 │
///    │   │                                 │
///    ▼   │         (u,v)                   │
///        │                                 │
///        │                                 │
///        │                                 │
///        │                                 │
///        └─────────────────────────────────┘

pub struct CubicFace2<'a> {
    points: [Point2; 4],
    face3: Option<&'a CubicFace3>,
    norm_a: f32,
    norm_b: f32,
}

impl<'a> Debug for CubicFace2<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "points: {:?}, {:?}, {:?}, {:?} ",
            self.points[0], self.points[1], self.points[2], self.points[3]
        )
    }
}

impl<'a> CubicFace2<'a> {
    pub fn new(points2d: [Point2; 4], face: &'a CubicFace3) -> Self {
        let points = face.points();
        let a = points[1] - points[0];
        let b = points[3] - points[0];
        Self {
            points: points2d,
            face3: Some(face),
            norm_a: a.norm(),
            norm_b: b.norm(),
        }
    }

    pub fn color_at_projection(&self, coordinates: &ProjectionCoordinates) -> &Color {
        let (u, v) = coordinates.to_uv(self.norm_a, self.norm_b);
        &self.face3.unwrap().texture().color_at(u, v)
    }

    /// Returns true if the face contains the given point
    ///
    /// TODO I am sure this can be optimized. For instance, we could keep a set
    ///      of all of the 2d pixels (it's a binary finite set) at creation of the
    ///      face and then just lookup in this set.
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

    /// Returns the raytracing distance (in mm, as u32) between the face and a ray defined as the pixels
    /// of the camera's screen, and the color of this pixel.
    ///
    /// Note: the distance is returned as u32 because f32 is not Orderable.
    pub fn raytracing(
        &self,
        u: i16,
        v: i16,
        camera: &Camera,
    ) -> Option<(u32, ProjectionCoordinates)> {

        if let Some(face) = self.face3 {
            // * v is in the referential of the camera frame
            // * c is in the referential of the world
            let direction = camera.ray_direction(u, v);
            let c = *camera.pose().position();
            return face.line_projection(&c, &direction);
        };
        None
    }

    pub fn distance_to(&self, cam: &Camera) -> f32 {
        self.face3.unwrap().distance_to(cam)
    }

    // Draws all of the pixels of self in the given frame
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            if self.contains(&Point2::new(x as f32, y as f32)) {
                // TODO: compute the correct projection
                let c = self
                    .color_at_projection(&ProjectionCoordinates::new(0., 0.))
                    .rgba();
                pixel.copy_from_slice(&c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cubic_face2::CubicFace2;
    use crate::primitives::cubic_face3::CubicFace3;
    use crate::primitives::point::Point2;
    use crate::primitives::position::Pose;
    use crate::primitives::textures::colored::ColoredTexture;
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
            face3: None,
            norm_a: 1.0,
            norm_b: 1.0,
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
            face3: None,
            norm_a: 1.0,
            norm_b: 1.0,
        };
        assert!(face2.contains(&Point2::new(161., 21.)));
    }

    #[test]
    /// Test that the raytracing algorithm works well
    fn raytracing() {
        // Create a camera
        let camera = Camera::new(
            Pose::new(Vector3::new(-2.0, 0., 0.), 0.0),
            100.0,
            100.,
            100.,
        );

        // Create a cubic2 face facing the camera
        let x: f32 = 0.;
        let y: f32 = -2.;
        let z: f32 = -2.;
        let face = CubicFace3::new(
            [
                Vector3::new(x, y, z),
                Vector3::new(x, y + 4., z),
                Vector3::new(x, y + 4., z + 4.),
                Vector3::new(x, y, z + 4.),
            ],
            Vector3::new(-1., 0., 0.),
            Box::new(ColoredTexture::new(Color::dark_blue())),
        );

        // Now let's get serious
        let projection = face.projection(&camera);
        println!("Projection = {projection:?}");

        let d1 = projection.raytracing(100, 100, &camera);
        let d1 = d1.unwrap().0;
        assert_eq!(d1, 2000);

        let d2 = projection.raytracing(110, 100, &camera).unwrap().0;
        let d3 = projection.raytracing(90, 100, &camera).unwrap().0;
        assert_eq!(d2, d3);
        assert!(d2 > d1);
        assert!(d3 > d1);

        let d4 = projection.raytracing(100, 110, &camera).unwrap().0;
        let d5 = projection.raytracing(100, 90, &camera).unwrap().0;
        assert_eq!(d4, d5);
        assert!(d4 > d1);
        assert!(d5 > d1);
    }
}
