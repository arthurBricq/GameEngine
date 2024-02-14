use crate::primitives::color::Color;

pub mod bw;
pub mod colored;
pub mod pixelized;

/// A texture is an interface that defines how to be rendered on the screen
pub trait Texture {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    /// Returns the color at the provided pixel coordinates, where
    /// u and v are expressed in the
    fn color_at(&self, u: f32, v: f32) -> &Color;
}
