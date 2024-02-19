/// Contains the projected coordinates (alpha, beta) such that a point P belonging to
/// a parallelogram can be written as
///
/// P = alpha * a + beta * b + P0
///
/// where
/// * P0 = first point of the parallelogram
/// * a = vector from P0 to P1
/// * b = vector from P0 to P3
#[derive(PartialEq, Debug)]
pub struct ProjectionCoordinates {
    alpha: f32,
    beta: f32,
}

impl ProjectionCoordinates {
    pub fn new(alpha: f32, beta: f32) -> Self {
        Self { alpha, beta }
    }

    pub fn none() -> Self {
        Self {
            alpha: 0.,
            beta: 0.,
        }
    }

    /// Returns coordinates in meters
    pub fn to_uv(&self, norm_a: f32, norm_b: f32) -> (f32, f32) {
        (self.alpha * norm_a, self.beta * norm_b)
    }
    pub fn alpha(&self) -> f32 {
        self.alpha
    }
    pub fn beta(&self) -> f32 {
        self.beta
    }

    pub fn is_inside_face(&self) -> bool {
        self.alpha >= 0. && self.alpha <= 1. && self.beta >= 0. && self.beta <= 1.
    }
}
