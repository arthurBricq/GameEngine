use std::ops::{Add, Sub};
use crate::primitives::vector::Vector3;

/// A point in 2D coordinates
#[derive(Copy, Clone, PartialEq)]
pub struct Point2 {
    x: f32,
    y: f32
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}