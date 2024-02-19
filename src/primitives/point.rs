use std::fmt::{Debug, Formatter};

/// A point in 2D coordinates
#[derive(Copy, Clone, PartialEq)]
pub struct Point2 {
    x: f32,
    y: f32,
    /// A flag to precise if the point is behind or in front of the camera
    in_front: Option<bool>
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y, in_front: None}
    }

    /// Creates a point by precising if the point is in front or behind the camera.
    pub fn new_with_direction(x: f32, y: f32, in_front: bool) -> Self {
        Self {x, y, in_front: Some(in_front)}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn in_front(&self) -> bool {
        self.in_front.unwrap_or(true)
    }
}

impl Debug for Point2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
