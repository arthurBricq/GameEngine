use winit::event::VirtualKeyCode;
use crate::drawable::Drawable;
use crate::{HEIGHT, WIDTH};

const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct BoxyWorld {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl BoxyWorld {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }
}

impl Drawable for BoxyWorld {
    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    fn left_mouse_pressed(&mut self, x: i16, y: i16) {
        self.box_x = x - BOX_SIZE;
        self.box_y = y - BOX_SIZE;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    fn key_pressed(&mut self, key: VirtualKeyCode) {
        todo!()
    }

    fn key_held(&mut self, key: VirtualKeyCode) {
        todo!()
    }
}
