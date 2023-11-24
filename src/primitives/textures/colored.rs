use crate::primitives::color::Color;
use crate::primitives::textures::Texture;

/// A simple texture which onl displays 1 color
pub struct ColoredTexture {
    color: Color
}

impl ColoredTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for ColoredTexture {
    fn width(&self) -> f32 {
        f32::MAX
    }

    fn height(&self) -> f32 {
        f32::MAX
    }

    fn color_at(&self, u: f32, v: f32) -> &Color {
        &self.color
    }
}