use std::collections::HashMap;
use std::usize;
use crate::primitives::color::Color;

use super::Texture;

pub struct Pixelated {
    rows: usize,
    cols: usize,
    pixel_size: f32,
    /// Holds the pattern of colors to use
    pixels: Vec<Vec<char>>,
    /// Holds the colors used by this texture
    colors: HashMap<char, Color>
}

impl Pixelated {

    pub fn new(lines: Vec<String>, pixel_size: f32) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let mut pixels = vec![];
        for line in lines {
            let row = line.chars().collect();
            pixels.push(row);
        }
        Self {
            rows,
            cols,
            pixel_size,
            pixels,
            colors: Color::create_colors_library()
        }
    }
}

impl Texture for Pixelated {
    fn width(&self) -> f32 {
        (self.rows as f32) * self.pixel_size
    }

    fn height(&self) -> f32 {
        (self.cols as f32) * self.pixel_size
    }

    fn color_at(&self, u: f32, v: f32) -> &crate::primitives::color::Color {
        // Compute the coordinates inside the primitive square
        let x = v % self.width();
        let y = u % self.height();
        // Compute the index in the array of pixels
        let i = (x / self.pixel_size) as usize;
        let j = (y / self.pixel_size) as usize;
        // Color matching
        self.colors.get(&self.pixels[i][j]).unwrap()
    }
}


impl Pixelated {

    /// A minecraft-like side of soil
    pub fn soil_side() -> Self {
        let lines = vec![
            "GGGGGGGGGG".to_string(),
            "GggggggggG".to_string(),
            "GggggggggG".to_string(),
            "WWWWWWWWWW".to_string(),
            "WwwwwwwwwW".to_string(),
            "WwwwwwwwwW".to_string(),
            "WwwwwwwwwW".to_string(),
            "WwwwwwwwwW".to_string(),
            "WwwwwwwwwW".to_string(),
            "WWWWWWWWWW".to_string(),
        ];
        return Pixelated::new(lines, 0.1);
    }

    pub fn soil_top() -> Self {
        let lines = vec![
            "GGGGGGGGGG".to_string(),
            "GggggggggG".to_string(),
            "GggGgggGgG".to_string(),
            "GggggggggG".to_string(),
            "GgGggggggG".to_string(),
            "GggggGgggG".to_string(),
            "GggGgggggG".to_string(),
            "GggggggGgG".to_string(),
            "GggggggggG".to_string(),
            "GGGGGGGGGG".to_string(),
        ];
        return Pixelated::new(lines, 0.1);
    }

    pub fn wood() -> Self {
        let lines = vec![
            "1111111111".to_string(),
            "1333333331".to_string(),
            "1222222221".to_string(),
            "1333333331".to_string(),
            "1333333331".to_string(),
            "1333333331".to_string(),
            "1333333331".to_string(),
            "1222222221".to_string(),
            "1333333331".to_string(),
            "1111111111".to_string(),
        ];
        return Pixelated::new(lines, 0.1);
    }

    pub fn wood_floor() -> Self {
        let lines = vec![
            "3333333333".to_string(),
            "3333333333".to_string(),
            "2222222222".to_string(),
            "3333333333".to_string(),
            "3333333333".to_string(),
            "3333333333".to_string(),
            "3333333333".to_string(),
            "2222222222".to_string(),
            "3333333333".to_string(),
            "3333333333".to_string(),
        ];
        return Pixelated::new(lines, 0.3);
    }

    pub fn stone() -> Self {
        let lines = vec![
            "4444444444".to_string(),
            "4656666664".to_string(),
            "4666665664".to_string(),
            "4665666664".to_string(),
            "4666666664".to_string(),
            "4665666664".to_string(),
            "4666666564".to_string(),
            "4656665664".to_string(),
            "4666666664".to_string(),
            "4444444444".to_string(),
        ];
        return Pixelated::new(lines, 0.1);
    }
}