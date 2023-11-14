use crate::drawable::Drawable;
use crate::primitives::cube::Cube3;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;

/// Representation of the world in 3D coordinates
/// A world simply contains several objects
pub struct World {
    objects: Vec<Cube3>,
    camera: Position,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            camera: Position::new(
                Vector3::new(0.0, 0.0, 0.0),
                0.0
            )
        }
    }

    pub fn add_object(&mut self, object: Cube3) {
        self.objects.push(object);
    }

    pub fn set_camera(&mut self, camera: Position) {
        self.camera = camera;
    }
}

impl Drawable for World {
    fn update(&mut self) {}

    fn draw(&self, _frame: &mut [u8]) {
        for object in &self.objects {
            // Get the lines
            let _faces = object.get_visible_faces(&self.camera);

            // Project each line in 2D
        }
    }

    fn left_mouse_pressed(&mut self, _x: i16, _y: i16) {}
}
