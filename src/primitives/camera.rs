use crate::{HEIGHT, WIDTH};
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
        Self {
            pose: position,
            f,
            px,
            py,
        }
    }

    pub fn default() -> Self {
        Self {
            pose: Pose::new(Vector3::empty(), 0.),
            f: 100.,
            px: WIDTH as f32 / 2.,
            py: HEIGHT as f32 / 2.
        }
    }

    /// Project the provided point (in world frame) into pixels
    pub fn project(&self, point: &Vector3) -> Point2 {
        // point is in frame references
        // https://www.brainvoyager.com/bv/doc/UsersGuide/CoordsAndTransforms/SpatialTransformationMatrices.html
        // We must transform `point` in the referential of the camera, and then apply the following
        // formula
        let transform = self.get_transform_world_to_cam();
        let point_in_cam_frame = transform.apply(point);
        // Transform the point in pixels using the formula
        // https://en.wikipedia.org/wiki/Camera_matrix#Normalized_camera_matrix_and_normalized_image_coordinates
        // In our case, the camera' forward direction is the x direction
        // Note: don't forget to take the absolute value of x, to not mess up points that are behind the camera.
        Point2::new_with_direction(
            self.f * point_in_cam_frame.y() / point_in_cam_frame.x().abs() + self.px,
            self.f * point_in_cam_frame.z() / point_in_cam_frame.x().abs() + self.py,
            point_in_cam_frame.x() > 0.0
        )
    }

    pub fn apply_z_rot(&mut self, rot: f32) {
        self.pose.apply_z_rot(rot)
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.pose.set_position(position);
    }

    pub fn set_rotation(&mut self, rot: f32) {
        self.pose.set_rotation(rot);
    }

    pub fn translate(&mut self, by: &Vector3) {
        self.pose.translate(by);
    }

    pub fn pose(&self) -> &Pose {
        &self.pose
    }

    pub fn orientation(&self) -> Vector3 {
        self.pose.orientation()
    }

    /// Returns a vector pointing in the direction of the ray directed by this pixel,
    /// in the camera frame
    pub fn ray_direction(&self, u: i16, v: i16) -> Vector3 {
        self.get_rotation_cam_to_world()
            * Vector3::new(
                1.0,
                (u as f32 - self.px) / self.f,
                (v as f32 - self.py) / self.f,
            )
    }

    pub fn is_point_visible(&self, point: &Vector3) -> bool {
        let uv = self.project(point);
        uv.in_front() && (uv.x() >= 0.0 || uv.x() < HEIGHT as f32) && (uv.y() >= 0.0 || uv.y() < WIDTH as f32)
    }
}

impl Camera {
    /// Returns a 3D transform that maps points in the world coordinates into camera coordinates
    fn get_transform_world_to_cam(&self) -> Transform {
        Transform::new(
            self.pose.position().opposite(),
            Matrix3::z_rotation(self.pose.rotation_z()),
        )
    }

    /// Returns a rotation matrix from cam coordinates to world coordinates
    fn get_rotation_cam_to_world(&self) -> Matrix3 {
        Matrix3::z_rotation(-self.pose.rotation_z())
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::camera::Camera;
    use crate::primitives::position::Pose;
    use crate::primitives::vector::Vector3;
    use std::f32::consts::PI;

    #[test]
    fn basic_projection() {
        // Create a point in the world frame
        let point_w = Vector3::empty();

        // Create a camera
        let mut cam = Camera::new(Pose::new(Vector3::new(1.0, 0.0, 0.0), 0.0), 1.0, 0.0, 0.0);

        // Compute the point in camera frame
        let transform = cam.get_transform_world_to_cam();
        let point_c = transform.apply(&point_w);

        // Compute the points in pixels
        println!("transform = {transform:?}");
        println!("point in camera frame: {:?}", point_c);

        cam.apply_z_rot(PI);

        let transform = cam.get_transform_world_to_cam();
        let point_c = transform.apply(&point_w);
        println!("transform = {transform:?}");
        println!("point in camera frame: {:?}", point_c);

        let uv = cam.project(&point_w);
        println!("pixels: {uv:?}");
    }
}
