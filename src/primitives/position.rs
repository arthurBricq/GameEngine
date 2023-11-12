use crate::primitives::point::Point3;
use crate::primitives::vector::Vector3;

pub struct Position {
    pos: Point3,
    orientation: Vector3,
}

impl Position {
    pub fn position(&self) -> &Point3 {
        &self.pos
    }

    pub fn orientation(&self) -> &Vector3 {
        &self.orientation
    }

    pub fn new(pos: Point3, orientation: Vector3) -> Self {
        Self { pos, orientation }
    }
}
