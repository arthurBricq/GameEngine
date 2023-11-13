use crate::primitives::vector::Vector3;

pub struct Position {
    pos: Vector3,
    orientation: Vector3,
}

impl Position {
    pub fn position(&self) -> &Vector3 {
        &self.pos
    }

    pub fn orientation(&self) -> &Vector3 {
        &self.orientation
    }

    pub fn new(pos: Vector3, orientation: Vector3) -> Self {
        Self { pos, orientation }
    }
}
