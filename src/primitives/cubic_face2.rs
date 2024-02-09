use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};

use crate::{HEIGHT, WIDTH};
use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::point::Point2;
use crate::primitives::projective_coordinates::ProjectionCoordinates;
use crate::primitives::textures::Texture;

/// A CubicFace2 is the projection of a CubicFace3 (is an oriented square in space)
///
/// Internal properties:
/// * face: A 2D face can hold a reference to its referring 3D face.
/// * the camera that observed this
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
    camera: &'a Camera
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
    pub fn new(points2d: [Point2; 4], face: &'a CubicFace3, camera: &'a Camera) -> Self {
        let points = face.points();
        let a = points[1] - points[0];
        let b = points[3] - points[0];
        Self {
            points: points2d,
            face3: Some(face),
            norm_a: a.norm(),
            norm_b: b.norm(),
            camera
        }
    }

    /// Returns the color at the given projection
    pub fn color_at_projection(&self, coordinates: &ProjectionCoordinates) -> &Color {
        let (u, v) = coordinates.to_uv(self.norm_a, self.norm_b);
        &self.face3.unwrap().texture().color_at(u, v)
    }

    /// Returns true if the face contains the given point
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
    ) -> Option<(u32, ProjectionCoordinates)> {
        if let Some(face) = self.face3 {
            // * v is in the referential of the camera frame
            // * c is in the referential of the world
            let direction = self.camera.ray_direction(u, v);
            let c = self.camera.pose().position();
            if let Some(proj) = face.line_projection(&c, &direction) {
                if proj.1.is_inside_face() {
                    return Some(proj);
                }
            }
        };
        None
    }

    pub fn distance_to(&self, cam: &Camera) -> f32 {
        self.face3.unwrap().distance_to(cam)
    }

    /// Returns a bounding box containing the box
    /// format: xmin, ymin, xmax, ymax
    fn bounding_box(&self) -> (u32, u32, u32, u32) {
        let mut xmin = self.points[0].x() as u32;
        let mut ymin = self.points[0].y() as u32;
        let mut xmax = self.points[0].x() as u32;
        let mut ymax = self.points[0].y() as u32;
        for i in 1..self.points.len() {
            let x = self.points[i].x() as u32;
            let y = self.points[i].y() as u32;
            xmin = min(x, xmin);
            ymin = min(y, ymin);
            xmax = max(x, xmax);
            ymax = max(y, ymax);
        }
        ((xmin-2).clamp(0, WIDTH), (ymin-2).clamp(0, HEIGHT), (xmax+2).clamp(0, WIDTH), (ymax+2).clamp(0, HEIGHT))
    }

    /// Draws all the pixels of self in the given frame.
    /// - TODO render proper color when working with textures
    pub fn draw(&self, frame: &mut [u8]) {
        /// Given a 2D position (in pixels), returns the index inside the 1D buffer of pixels.
        fn pos_to_index(x: u32, y: u32) -> usize {
            4 * (x + y * WIDTH) as usize
        }

        let (xmin, ymin, xmax, ymax) = self.bounding_box();
        let mut x = xmin;
        let mut y = ymin;

        // go through all the points in the bounding box
        while y < ymax {
            while x < xmax {
                if self.contains(&Point2::new(x as f32, y as f32)) {
                    if let Some((_, projection)) = self.raytracing(x as i16, y as i16) {
                        let i = pos_to_index(x, y);
                        let pixel = &mut frame[i..i+4];
                        let c = self.color_at_projection(&projection).rgba();
                        pixel.copy_from_slice(&c);
                    }
                }
                x += 1;
            }
            x = xmin;
            y += 1;
        }
    }

    // Returns true if the faces are roughly equals.
    pub fn equals_to(&self, other: &CubicFace2) -> bool {
        self.points == other.points
    }
    pub fn points(&self) -> [Point2; 4] {
        self.points.clone()
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
            camera: &Camera::default()
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
            camera: &Camera::default()
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

        let d1 = projection.raytracing(100, 100);
        let d1 = d1.unwrap().0;
        assert_eq!(d1, 2000);

        let d2 = projection.raytracing(110, 100).unwrap().0;
        let d3 = projection.raytracing(90, 100).unwrap().0;
        assert_eq!(d2, d3);
        assert!(d2 > d1);
        assert!(d3 > d1);

        let d4 = projection.raytracing(100, 110).unwrap().0;
        let d5 = projection.raytracing(100, 90).unwrap().0;
        assert_eq!(d4, d5);
        assert!(d4 > d1);
        assert!(d5 > d1);
    }
}
