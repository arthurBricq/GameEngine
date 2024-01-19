use std::time::Instant;
use pixels::{Pixels, SurfaceTexture};
use winit::event::VirtualKeyCode;
use winit::window::Window;

use crate::drawable::Drawable;
use crate::motion_model::{MotionModel, DEFAULT_ACC};
use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face2::{CubicFace2, ProjectionCoordinates};
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::object::Object;
use crate::primitives::point::Point2;
use crate::primitives::vector::Vector3;
use crate::{HEIGHT, WIDTH};

/// Representation of the world in 3D coordinates
/// A world simply contains several objects
pub struct World {
    objects: Vec<Box<dyn Object>>,
    camera: Camera,
    /// The motion model is the class responsible for smoothly updating the position
    motion_model: MotionModel,
    /// Keep track for each time intervals to correctly update the motion model
    last_time: Instant,
    /// At each iteration, keep track whether a motion was applied
    motion_applied: bool,
}

impl World {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
            motion_model: MotionModel::new(),
            last_time: Instant::now(),
            motion_applied: false,
        }
    }

    pub fn add_cube(&mut self, cube: Cube3) {
        self.objects.push(Box::new(cube));
    }

    pub fn add_face(&mut self, face: CubicFace3) {
        self.objects.push(Box::new(face));
    }

    pub fn set_camera_position(&mut self, position: Vector3) {
        self.camera.set_position(position);
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Debug function
    pub fn save_current_image(&self) {
        // TODO: look this up
        // https://stackoverflow.com/a/38956995/13219173
    }
}

impl Drawable for World {
    fn draw_painter(&self, frame: &mut [u8]) {
        // Find the faces that are visible to the camera's perspective
        let mut faces2: Vec<CubicFace2> = Vec::new();
        for object in &self.objects {
            let faces = object.get_visible_faces(&self.camera);
            for face in faces {
                let face2d = face.projection(&self.camera);
                faces2.push(face2d);
            }
        }

        // Sort the faces by depth, from the farthest polygon to the closest polygon
        // The sorting iis done over i32, because f32 does not implements Ord.
        faces2.sort_by_key(|f| (f.distance_to(&self.camera) * 1000.) as i32);

        // Draw the background color
        let background = [214, 214, 194, 150];
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            pixel.copy_from_slice(&background);
        }

        // Paint the pixels, starting from the most distant ones
        faces2.iter().rev().for_each(|f| f.draw(frame));
    }

    fn draw(&self, frame: &mut [u8]) {
        // Find the faces that are visible to the camera's perspective
        let mut faces2: Vec<CubicFace2> = Vec::new();
        for object in &self.objects {
            // Get the visible 3d faces
            let faces = object.get_visible_faces(&self.camera);
            // For each face, perform a 2d projection on the camera frame
            for face in faces {
                let face2d = face.projection(&self.camera);
                faces2.push(face2d);
            }
        }

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            // For each pixel, find
            // * the closest face
            // * the coordinate (in the frame's reference) of the raytracing intersection
            let mut min_distance = u32::MAX;
            let mut best_projection: Option<ProjectionCoordinates> = None;
            let mut best_face: Option<&CubicFace2> = None;
            for f2 in &faces2 {
                if f2.contains(&Point2::new(x as f32, y as f32)) {
                    if let Some(projection) = f2.raytracing(x, y, &self.camera) {
                        if projection.0 < min_distance {
                            min_distance = projection.0;
                            best_face = Some(f2);
                            best_projection = Some(projection.1)
                        }
                    }
                }
            }

            // find the first face of this point (if it exists)
            let rgba = if let Some(face) = best_face {
                face.color_at_projection(&best_projection.unwrap()).rgba()
            } else {
                [214, 214, 194, 150]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    fn left_mouse_pressed(&mut self, _x: i16, _y: i16) {
        println!("{_x}, {_y}");
    }

    fn key_pressed(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::R => {
                // Rotate the camera's
                self.camera.apply_z_rot(std::f32::consts::PI / 16.);
            }
            VirtualKeyCode::E => {
                self.camera.apply_z_rot(-std::f32::consts::PI / 16.);
            }
            _ => {}
        }
    }

    fn key_held(&mut self, key: VirtualKeyCode) {
        self.motion_applied = true;
        match key {
            VirtualKeyCode::Up => self
                .motion_model
                .increment_direction(self.camera.orientation(), DEFAULT_ACC),
            VirtualKeyCode::Down => self
                .motion_model
                .increment_direction(self.camera.orientation().opposite(), DEFAULT_ACC),
            VirtualKeyCode::Right => self
                .motion_model
                .increment_direction(self.camera.orientation().anticlockwise(), DEFAULT_ACC),
            VirtualKeyCode::Left => self
                .motion_model
                .increment_direction(self.camera.orientation().clockwise(), DEFAULT_ACC),
            VirtualKeyCode::J => self.motion_model.apply(2, DEFAULT_ACC),
            VirtualKeyCode::K => self.motion_model.apply(2, -DEFAULT_ACC),
            _ => {}
        }
    }

    /// Update is called at the end of each UI loop, right before rendering the screen
    /// and calling the `draw` function.
    fn update(&mut self) {
        let elapsed = self.last_time.elapsed();
        self.last_time = Instant::now();

        // Obstacle detection

        // If no key was pressed, slow down the motion
        if !self.motion_applied {
            self.motion_model.slow_down();
        }

        // Update the camera position using the motion model
        self.camera.set_position(
            self.motion_model
                .new_pos(self.camera.pose().position(), elapsed.as_secs_f32()),
        );

        // reset the temporary variable
        self.motion_applied = false;
    }
}
