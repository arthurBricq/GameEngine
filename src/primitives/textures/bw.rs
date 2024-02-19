use crate::primitives::color::Color;
use crate::primitives::textures::Texture;

#[derive(Clone)]
pub struct BWTexture {
    w: f32,
    h: f32,
    colors: [Color; 2],
}

impl BWTexture {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            w,
            h,
            colors: [Color::white(), Color::black()],
        }
    }
}

impl Texture for BWTexture {
    fn width(&self) -> f32 {
        self.w
    }

    fn height(&self) -> f32 {
        self.h
    }

    fn color_at(&self, u: f32, v: f32) -> &Color {
        let x = u % self.w;
        let y = v % self.h;
        let w2 = self.w / 2.;
        let h2 = self.w / 2.;
        match (x, y) {
            (a, b) if a <= w2 && b <= h2 => &self.colors[0],
            (a, b) if a >= w2 && b >= h2 => &self.colors[0],
            (a, b) if a <= w2 && b >= h2 => &self.colors[1],
            (a, b) if a >= w2 && b <= h2 => &self.colors[1],
            _ => panic!("(x,y) should never not be covered"),
        }
    }
}
