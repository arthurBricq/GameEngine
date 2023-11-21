use crate::primitives::vector::Vector3;

pub struct Pose {
    pos: Vector3,
    // for now, we only assume that there is a rotation in the z-axis
    rotz: f32,
}

impl Pose {
    pub fn position(&self) -> &Vector3 {
        &self.pos
    }

    pub fn rotation_z(&self) -> f32 {
        self.rotz
    }

    pub fn orientation(&self) -> Vector3 {
        Vector3::new(f32::cos(self.rotz), f32::sin(self.rotz), 0.0)
    }

    pub fn new(pos: Vector3, rotz: f32) -> Self {
        Self { pos, rotz }
    }

    pub fn apply_z_rot(&mut self, rot: f32) {
        self.rotz += rot;
    }

    pub fn translate(&mut self, by: &Vector3) {
        self.pos = self.pos + *by
    }

    pub fn set_position(&mut self, pos: Vector3) {
        self.pos = pos;
    }
}
