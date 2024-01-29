use crate::primitives::matrix3::Matrix3;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub};

/// A vector in 3 coordinates
///
/// Mathematically, it can represent equally a 3d vector or a 3d point
#[derive(Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a face from integers
    pub fn newi(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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

    /// Returns the vector defined by:
    ///    v = other - self
    /// eg the vector going from self to other
    pub fn line_to(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_line_to() {
        let p0 = Vector3::new(0., 0., 0.);
        let p1 = Vector3::new(1., 1., 1.);
        let v01 = p0.line_to(&p1);
        assert_eq!(v01.x, 1.0);
        assert_eq!(v01.y, 1.0);
        assert_eq!(v01.z, 1.0);
        let v10 = p1.line_to(&p0);
        assert_eq!(v10.x, -1.0);
        assert_eq!(v10.y, -1.0);
        assert_eq!(v10.z, -1.0);
    }
}
/// Math operations

impl Vector3 {
    /// Dot product with another vector
    pub fn dot(&self, vec: &Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    /// Returns a vector in the opposite direction
    pub fn opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Returns a vector rotated 90 degrees clockwise around the z-axis
    pub fn clockwise(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    /// Returns a vector rotated 90 degrees anticlockwise around the z-axis
    pub fn anticlockwise(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }

    pub fn clamp(&mut self, min: f32, max: f32) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
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

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
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

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
