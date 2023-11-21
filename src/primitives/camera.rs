use crate::primitives::matrix3::Matrix3;
use crate::primitives::point::Point2;
use crate::primitives::position::Pose;
use crate::primitives::transformation::Transform;
use crate::primitives::vector::Vector3;

/// A camera is a position and calibration parameters
pub struct Camera {
    pose: Pose,
    f: f32,
    px: f32,
    py: f32,
}

impl Camera {
    pub fn new(position: Pose, f: f32, px: f32, py: f32) -> Self {
        Self { pose: position, f, px, py }
    }

    /// Project the provided point (in world frame) into pixels
    pub fn project(&self, point: Vector3) -> Point2 {
        // point is in frame references
        // https://www.brainvoyager.com/bv/doc/UsersGuide/CoordsAndTransforms/SpatialTransformationMatrices.html
        // We must transform `point` in the referential of the camera, and then apply the following
        // formula
        let transform = self.get_transform();
        let point_in_cam_frame = transform.apply(point);
        // Transform the point in pixels using the formula
        // https://en.wikipedia.org/wiki/Camera_matrix#Normalized_camera_matrix_and_normalized_image_coordinates
        //
        // In our case, the camera' forward direction
        Point2::new(
            self.f * point_in_cam_frame.y() / point_in_cam_frame.x() + self.px,
            self.f * point_in_cam_frame.z() / point_in_cam_frame.x() + self.py,
        )
    }

    pub fn apply_z_rot(&mut self, rot: f32) {
        self.pose.apply_z_rot(rot)
    }

    pub fn set_position(&mut self, position: Vector3) {
        println!("new pos = {position:?}");
        self.pose.set_position(position);
    }

    pub fn position(&self) -> &Pose {
        &self.pose
    }

    pub fn translate(&mut self, by: &Vector3) {
        self.pose.translate(by);
    }

    pub fn ray_direction(&self, u: i16, v: i16) -> Vector3 {
        Vector3::new(1.0, (u as f32 - self.px) / self.f, (v as f32 - self.py) / self.f)
    }
}

impl Camera {
    fn get_transform(&self) -> Transform {
        Transform::new(self.pose.position().opposite(),
                       Matrix3::z_rotation(self.pose.rotation_z()),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::primitives::camera::Camera;
    use crate::primitives::position::Pose;
    use crate::primitives::vector::Vector3;

    #[test]
    fn basic_projection() {
        // Create a point in the world frame
        let point_w = Vector3::empty();

        // Create a camera
        let mut cam = Camera::new(
            Pose::new(
                Vector3::new(1.0, 0.0, 0.0),
                0.0,
            ),
            1.0,
            0.0,
            0.0,
        );

        // Compute the point in camera frame
        let transform = cam.get_transform();
        let point_c = transform.apply(point_w);

        // Compute the points in pixels
        println!("transform = {transform:?}");
        println!("point in camera frame: {:?}", point_c);

        cam.apply_z_rot(PI);

        let transform = cam.get_transform();
        let point_c = transform.apply(point_w);
        println!("transform = {transform:?}");
        println!("point in camera frame: {:?}", point_c);

        let uv = cam.project(point_w);
        println!("pixels: {uv:?}");
    }
}