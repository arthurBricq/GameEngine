use winit::dpi::Position;
use crate::primitives::point::{Point2, Point3};

pub struct Camera {
    position: Position,
    f: f32,
    px: f32,
    py: f32,
}

impl Camera {
    pub fn new(position: Position, f: f32, px: f32, py: f32) -> Self {
        Self { position, f, px, py }
    }

    pub fn project(&self, point: Point3) -> Point2 {
        // TODO point is in frame references
        // We must transform it into

        Point2::new(
            self.f * point.x() + self.px * point.z(),
            self.f * point.y() + self.py * point.z(),
        )
    }
}