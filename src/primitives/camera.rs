use crate::primitives::matrix3::Matrix3;
use crate::primitives::point::Point2;
use crate::primitives::position::Position;
use crate::primitives::transformation::Transform;
use crate::primitives::vector::Vector3;

/// A camera is a position and calibration parameters
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

    /// Project the provided point (in world frame) into pixels
    pub fn project(&self, point: Vector3) -> Point2 {
        // TODO point is in frame references
        // https://www.brainvoyager.com/bv/doc/UsersGuide/CoordsAndTransforms/SpatialTransformationMatrices.html
        // We must transform `point` in the referential of the camera, and then apply the following
        // formula
        let transform = self.get_transform();
        let cam_frame = transform.apply(point);
        Point2::new(
            self.f * cam_frame.x() + self.px * cam_frame.z(),
            self.f * cam_frame.y() + self.py * cam_frame.z(),
        )
    }
}

impl Camera {
    fn get_transform(&self) -> Transform {
        Transform::new(*self.position.position(),
                       Matrix3::z_rotation(self.position.rotation_z())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    #[test]
    fn basic_projection() {
        // Create a point in the world frame
        let point_w = Vector3::empty();

        // Create a camera
        let cam = Camera::new(
            Position::new(
                Vector3::new(1.0, 0.0, 0.0),
                0.0
            ),
            1.0,
            0.0,
            0.0
        );

        let transform = cam.get_transform();

        // Compute the points in pixels


    }

}