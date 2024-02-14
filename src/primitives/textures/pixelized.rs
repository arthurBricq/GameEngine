use std::usize;

use super::Texture;

pub struct Pixelized {
    rows: usize,
    cols: usize,
}

impl Pixelized {
    pub fn new(content: &String) -> Self {
        let lines: Vec<&str> = content.lines().collect();
        Self {
            rows: lines.len(),
            cols: 0,
        }
    }
}

impl Texture for Pixelized {
    fn width(&self) -> f32 {
        todo!()
    }

    fn height(&self) -> f32 {
        todo!()
    }

    fn color_at(&self, u: f32, v: f32) -> &crate::primitives::color::Color {
        todo!()
    }
}
