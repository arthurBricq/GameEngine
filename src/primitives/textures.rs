pub mod bw;
pub mod colored;

use dyn_clone::{clone_trait_object, DynClone};
use crate::primitives::color::Color;

/// A texture is an interface that defines how to be rendered on the screen
pub trait Texture : DynClone {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    /// Returns the color at the provided pixel coordinates, where
    /// u and v are expressed in the
    fn color_at(&self, u: f32, v: f32) -> &Color;
}

clone_trait_object!(Texture);