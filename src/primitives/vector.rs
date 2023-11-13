use std::ops::{Add, Sub};

/// A vector in 3 coordinates
///
/// Mathematically, it can represent equally a 3d vector or a 3d point
#[derive(Clone, Copy)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

/// Math operations

impl Vector3 {

    /// Dot product with another vector
    pub fn dot(&self, vec: &Vector3) -> f32 {
        self.x * vec.x + self.y + vec.y + self.z * vec.z
    }

    /// Returns a vector in the opposite direction
    pub fn opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    /// Returns a vector rotated 90 degrees clockwise around the z-axis
    pub fn clockwise(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z
        }
    }

    /// Returns a vector rotated 90 degrees anticlockwise around the z-axis
    pub fn anticlockwise(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
            z: self.z
        }
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
