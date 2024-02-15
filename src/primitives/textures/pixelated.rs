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
    fn create_colors_library() -> HashMap<char, Color> {
        let mut colors = HashMap::new();
        colors.insert('y', Color::yellow());
        colors.insert('b', Color::dark_blue());
        colors.insert('k', Color::black());
        colors.insert('0', Color::white());
        colors.insert('t', Color::turquoise());
        colors.insert('g', Color::green());
        colors.insert('G', Color::dark_green());
        colors.insert('w', Color::brown1());
        colors.insert('W', Color::brown2());
        colors.insert('o', Color::orange());
        colors
    }

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
            colors: Self::create_colors_library()
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
    pub fn test1() -> Self {
        let lines = vec![
            "yyy".to_string(),
            "bbb".to_string()
        ];
        return Pixelated::new(lines, 0.15);
    }

    pub fn test2() -> Self {
        let lines = vec![
            "gGo".to_string(),
            "Gob".to_string(),
            "obg".to_string()
        ];
        return Pixelated::new(lines, 0.15);
    }

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
            "WWWWWWWWWW".to_string(),
        ];
        return Pixelated::new(lines, 0.1);
    }
}