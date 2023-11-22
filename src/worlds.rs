use std::time::Instant;
use winit::event::VirtualKeyCode;

use crate::drawable::Drawable;
use crate::motion_model::{DEFAULT_ACC, MotionModel};
use crate::primitives::camera::Camera;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face2::CubicFace2;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::object::Object;
use crate::primitives::point::Point2;
use crate::primitives::vector::Vector3;
use crate::WIDTH;

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
    motion_applied: bool
}

impl World {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
            motion_model: MotionModel::new(),
            last_time: Instant::now(),
            motion_applied: false
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
}

impl Drawable for World {


    fn draw(&self, frame: &mut [u8]) {
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

            // Check if the point is contained in a face
            let contained = faces2.iter().any(|face2| face2.contains(&Point2::new(x as f32, y as f32)));

            // TODO : find the closest face intersecting with the ray

            let closest_face = faces2
                .iter()
                .filter(|face2| face2.contains(&Point2::new(x as f32, y as f32)))
                // .enumerate()
                .min_by_key(|face2|
                    if let Some(d) = face2.raytracing_distance(x, y, &self.camera) {
                        (d * 1000.) as u32
                    } else {
                        std::u32::MAX
                    }
                );

            // find the first face of this point (if it exists)
            let rgba = if let Some(face) = closest_face {
                face.color().rgba()
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
                for o in &mut self.objects {
                    o.rotate(std::f32::consts::PI / 16.);
                }
            },
            VirtualKeyCode::E => {
                for o in &mut self.objects {
                    o.rotate(-std::f32::consts::PI / 16.);
                }
            },
            _ => {}
        }
    }

    fn key_held(&mut self, key: VirtualKeyCode) {
        self.motion_applied = true;
        match key {
            VirtualKeyCode::Up => self.motion_model.apply(0, DEFAULT_ACC),
            VirtualKeyCode::Down => self.motion_model.apply(0, -DEFAULT_ACC),
            VirtualKeyCode::Right => self.motion_model.apply(1, DEFAULT_ACC),
            VirtualKeyCode::Left => self.motion_model.apply(1, -DEFAULT_ACC),
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

        // If no key was pressed, slow down the motion
        if !self.motion_applied {
            self.motion_model.slow_down();
        }

        // Update the camera position using the motion model
        self.camera.set_position(
            self.motion_model.new_pos(self.camera.position().position(), elapsed.as_secs_f32())
        );

        // reset the temporary variable
        self.motion_applied = false;
    }
}
