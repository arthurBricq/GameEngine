use winit::event::VirtualKeyCode;

/// If an object is drawable, it can be rendered onto the screen
pub trait Drawable {
    /// Update the internal state
    fn update(&mut self);

    /// Draw onto pixels
    fn draw(&self, frame: &mut [u8]);

    /// Draw using the painter' algorithm
    fn draw_painter(&self, frame: &mut [u8]);

    /// Called when the left mouse button is pressed at a given position of the screen
    fn left_mouse_pressed(&mut self, x: i16, y: i16);

    /// A key was pressed
    fn key_pressed(&mut self, key: VirtualKeyCode);

    /// A key as held (maintaining press)
    fn key_held(&mut self, key: VirtualKeyCode);
}
