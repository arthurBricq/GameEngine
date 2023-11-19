use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};

#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgba(rgba: [u8; 4]) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        }
    }

    pub fn purple() -> Self {
        Self {r: 255, g: 0, b: 255, a: 255}
    }

    pub fn yellow() -> Self {
        Self {r: 255, g: 255, b: 0, a: 255}
    }

    pub fn dark_blue() -> Self {
        Self {r: 0, g: 0, b: 153, a: 255}
    }

    pub fn white() -> Self {
        Self {r: 0, g: 0, b: 0, a: 255}
    }

    pub fn black() -> Self {
        Self {r: 255, g: 255, b: 255, a: 255}
    }

    // Randomize one of the axis of this color, between 0 and 255.
    // The axis are: 0=r, 1=g, 2=b, 3=a
    // eg, if axis = 3, randomize the alpha b
    pub fn randomize_dimension(&self, axis: usize) -> Self {
        let between = Uniform::from(0u8..255);
        let mut rng = rand::thread_rng();
        // let i1 = between.sample(&mut rng) as u8;
        let x1 = between.sample(&mut rng);
        match axis {
            0 => Self { r: x1, g: self.g, b: self.b, a: self.a},
            1 => Self { r: self.r, g: x1, b: self.b, a: self.a},
            2 => Self { r: self.r, g: self.g, b: x1, a: self.a},
            3 => Self { r: self.r, g: self.g, b: self.b, a: x1},
            _ => self.clone()
        }
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}