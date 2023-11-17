use winit::event::VirtualKeyCode;
use crate::drawable::Drawable;
use crate::primitives::camera::Camera;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face::CubicFace2;
use crate::primitives::point::Point2;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::WIDTH;

/// Representation of the world in 3D coordinates
/// A world simply contains several objects
pub struct World {
    objects: Vec<Cube3>,
    camera: Camera,
}

impl World {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
        }
    }

    pub fn add_object(&mut self, object: Cube3) {
        self.objects.push(object);
    }

    pub fn set_camera_position(&mut self, camera: Position) {
        self.camera.set_position(camera);
    }
}

impl Drawable for World {
    fn update(&mut self) {}

    fn draw(&self, frame: &mut [u8]) {
        let mut faces2: Vec<CubicFace2> = Vec::new();

        for object in &self.objects {
            // Get the visible 3d faces
            let faces = object.get_visible_faces(&self.camera);
            // For each face, perform a 2d projection on the camera frame
            for face in faces {
                let face2d = face.projection(&self.camera);
                faces2.push(face2d);
                // println!("3D face: = {face:?}");
                // println!("2D face: = {face2d:?}");
            }
        }


        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            // Check if the point is contained in a face
            let contained = faces2.iter().any(|face2| face2.contains(&Point2::new(x as f32, y as f32)));

            // find the first face of this point (if it exists)
            let rgba = if let Some(face) = faces2
                .iter().find(|face2| face2.contains(&Point2::new(x as f32, y as f32))) {
                face.color().rgba()
            } else {
               [214, 214, 194, 150]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    fn left_mouse_pressed(&mut self, _x: i16, _y: i16) {}

    fn key_pressed(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up => self.camera.translate(&Vector3::new(0.1, 0.0, 0.0)),
            VirtualKeyCode::Down => self.camera.translate(&Vector3::new(-0.1, 0.0, 0.0)),
            VirtualKeyCode::Right => self.camera.translate(&Vector3::new(0.0, 0.1, 0.0)),
            VirtualKeyCode::Left => self.camera.translate(&Vector3::new(0.0, -0.1, 0.0)),
            VirtualKeyCode::J => self.camera.translate(&Vector3::new(0.0, 0.0, -0.1)),
            VirtualKeyCode::K => self.camera.translate(&Vector3::new(0.0, 0.0, 0.1)),
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
}
