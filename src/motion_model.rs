use crate::primitives::vector::Vector3;

const MAX_ACC: f32 = 50.;
const DEFAULT_DEC: f32 = 0.5;
pub const DEFAULT_ACC: f32 = 100.;

pub struct MotionModel {
    ax: f32,
    ay: f32,
    az: f32
}

impl MotionModel {
    pub fn new() -> Self {
        Self { ax: 0., ay: 0., az: 0. }
    }

    /// Returns the position updated by the motion model
    pub fn new_pos(&self, pos: &Vector3, dt: f32) -> Vector3 {
        //
        // self.apply(0, if self.ax > 0 )

        *pos + Vector3::new(
            self.ax * dt * dt,
            self.ay * dt * dt,
            self.az * dt * dt,
        )
    }

    pub fn apply(&mut self, axis: usize, inc: f32) {
        println!("applying {inc} on {axis} -> {}, {}, {}", self.ax, self.ay, self.az );
        match axis {
            0 => self.ax = (self.ax + inc).clamp(-MAX_ACC, MAX_ACC),
            1 => self.ay = (self.ay + inc).clamp(-MAX_ACC, MAX_ACC),
            2 => self.ax = (self.ax + inc).clamp(-MAX_ACC, MAX_ACC),
            _ => {}
        }
    }
}