use winit::dpi::Position;
use crate::primitives::point::Point2;
use crate::primitives::vector::Vector3;

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

    pub fn project(&self, point: Vector3) -> Point2 {
        // TODO point is in frame references
        // https://www.brainvoyager.com/bv/doc/UsersGuide/CoordsAndTransforms/SpatialTransformationMatrices.html
        // We must transform `point` in the referential of the camera, and then apply the following
        // formula

        Point2::new(
            self.f * point.x() + self.px * point.z(),
            self.f * point.y() + self.py * point.z(),
        )
    }
}

impl Camera {
    fn transformation_to_camera(&self) {

    }
}