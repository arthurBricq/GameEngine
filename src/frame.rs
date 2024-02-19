use crate::primitives::cubic_face2::CubicFace2;
use crate::primitives::cubic_face3::CubicFace3;

/// A frame is an object able to draw faces
pub trait AbstractFrame {
    /// Draws the given 2D polygon onto the screen
    fn draw_one_face(&mut self, face: &CubicFace2);
}

pub struct Frame<'a> {
    buffer: &'a mut [u8]
}

impl<'a> Frame<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer }
    }
}

impl<'a> AbstractFrame for Frame<'a> {
    fn draw_one_face(&mut self, face: &CubicFace2) {
        face.draw(self.buffer);
    }
}