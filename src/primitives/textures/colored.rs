use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::primitives::color::Color;
use crate::primitives::textures::Texture;

/// A simple texture which onl displays 1 color
#[derive(Clone)]
pub struct ColoredTexture {
    color: Color,
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


// Define most basic textures as static variables
pub static YELLOW: Lazy<Mutex<ColoredTexture>> = Lazy::new(|| {
    Mutex::new(ColoredTexture::new(Color::yellow()))
});
pub static BLACK: ColoredTexture = ColoredTexture::new(Color::black());
pub static PURPLE: ColoredTexture = ColoredTexture::new(Color::purple());
pub static ORANGE: ColoredTexture = ColoredTexture::new(Color::orange());
