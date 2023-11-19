use std::fmt::{Debug, Formatter};
use crate::primitives::color::Color;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::point::Point2;

/// A cubic face is an oriented square in space.
/// A 2D face can hold a referenc
pub struct CubicFace2<'a> {
    points: [Point2; 4],
    color: Color,
    face3: Option<&'a CubicFace3>
}

impl <'a> Debug for CubicFace2<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "points: {:?}, {:?}, {:?}, {:?} ", self.points[0], self.points[1], self.points[2], self.points[3])
    }
}

impl <'a> CubicFace2 <'a> {

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
        return (c1 == c2) && (c1 == c3) && (c1 == c4)
    }

    pub fn distance_to_point() -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::color::Color;
    use crate::primitives::cubic_face2::CubicFace2;
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
            face3: None
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
            face3: None
        };

        assert!(face2.contains(&Point2::new(161., 21.)));
    }
}
