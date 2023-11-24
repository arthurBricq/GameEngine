pub mod colored;

use crate::primitives::color::Color;

/// A texture is an interface that defines how to be rendered on the screen
pub trait Texture {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn color_at(&self, u: f32, v: f32) -> &Color;
}


/// A black and white texture
pub struct BWTexture {

}