use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
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
            0 => Self {
                r: x1,
                g: self.g,
                b: self.b,
                a: self.a,
            },
            1 => Self {
                r: self.r,
                g: x1,
                b: self.b,
                a: self.a,
            },
            2 => Self {
                r: self.r,
                g: self.g,
                b: x1,
                a: self.a,
            },
            3 => Self {
                r: self.r,
                g: self.g,
                b: self.b,
                a: x1,
            },
            _ => self.clone(),
        }
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_rgba(rgba: [u8; 4]) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        }
    }

    pub const fn purple() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub const fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub const fn dark_blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 153,
            a: 255,
        }
    }

    pub const fn red() -> Self {
        Self {
            r: 255,
            g: 51,
            b: 51,
            a: 255,
        }
    }

    pub const fn orange() -> Self {
        Self {
            r: 255,
            g: 153,
            b: 51,
            a: 255,
        }
    }

    pub const fn light_green() -> Self {
        Self {
            r: 153,
            g: 255,
            b: 51,
            a: 255,
        }
    }


    pub const fn turquoise() -> Self {
        Self {
            r: 102,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub const fn white() -> Self {
        Self::new(0,0,0,255)
    }

    pub const fn black() -> Self {
        Self::new(255, 255, 255, 255)
    }

    pub const fn brown1() -> Self {
        Self::new(106, 81, 47, 255)
    }

    pub const fn brown2() -> Self {
        Self::new(100, 74, 44, 255)
    }

    pub const fn brown3() -> Self {
        Self::new(90, 62, 34, 255)
    }

    pub const fn dark_green() -> Self {
        Self::new(67, 95, 42, 255)
    }

    pub const fn green() -> Self {
        Self::new(90, 123, 57, 255)
    }

    pub const fn wood_dark() -> Self {
        Self::new(139,111,66,255)
    }

    pub const fn wood() -> Self {
        Self::new(155,127,75,255)
    }

    pub const fn wood_light() -> Self {
        Self::new(174,142,85,255)
    }

    pub const fn stone_dark() -> Self {
        Self::new(101,101,101,255)
    }

    pub const fn stone() -> Self {
        Self::new(115,115,115,255)
    }

    pub const fn stone_light() -> Self {
        Self::new(127,127,127,255)
    }

    pub fn create_colors_library() -> HashMap<char, Color> {
        let mut colors = HashMap::new();
        colors.insert('y', Color::yellow());
        colors.insert('b', Color::dark_blue());
        colors.insert('k', Color::black());
        colors.insert('0', Color::white());
        colors.insert('t', Color::turquoise());
        colors.insert('o', Color::orange());
        // Use for minecraft-like
        colors.insert('g', Color::green());
        colors.insert('G', Color::dark_green());
        colors.insert('w', Color::brown1());
        colors.insert('W', Color::brown2());
        colors.insert('1', Color::wood_dark());
        colors.insert('2', Color::wood());
        colors.insert('3', Color::wood_light());
        colors.insert('4', Color::stone_dark());
        colors.insert('5', Color::stone());
        colors.insert('6', Color::stone_light());
        colors
    }
}
