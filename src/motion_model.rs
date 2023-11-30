use crate::primitives::vector::Vector3;

const MAX_ACC: f32 = 50.;
const MIN_ACC: f32 = 10.;
pub const DEFAULT_ACC: f32 = 100.;

pub struct MotionModel {
    acc: Vector3
}

impl MotionModel {
    pub fn new() -> Self {
        Self {acc: Vector3::empty()}
    }

    /// Returns the position updated by the motion model
    pub fn new_pos(&mut self, pos: &Vector3, dt: f32) -> Vector3 {
        *pos + (self.acc * dt * dt)
    }

    pub fn slow_down(&mut self) {
        // Apply motions to come back to still state
        self.slow_down_axis(0);
        self.slow_down_axis(1);
        self.slow_down_axis(2);
    }

    fn slow_down_axis(&mut self, axis: usize) {
        if self.acc[axis] > MIN_ACC {
            let correction = if self.acc[axis] > 0. { -self.acc[axis] * 0.3 } else { self.acc[axis] * 0.3 };
            self.apply(axis, correction)
        } else {
            self.acc[axis] = 0.
        }
    }

    pub fn apply(&mut self, axis: usize, inc: f32) {
        self.acc[axis] = (self.acc[axis] + inc).clamp(-MAX_ACC, MAX_ACC)
    }

    pub fn increment_direction(&mut self, axis: Vector3, inc: f32) {
        println!("{axis:?}");
        self.acc += axis * inc;
        self.acc.clamp(-MAX_ACC, MAX_ACC)
    }
}