use crate::bsp::cubic_face_split::{bsp_polygon_split, point_in_front_of};
use crate::frame::AbstractFrame;
use crate::primitives::camera::Camera;
use crate::primitives::cubic_face3::CubicFace3;

use std::ops::Deref;

/// Binary Space Partionning
///
/// This class represents a binary tree in rust
pub struct BSPNode {
    faces: Vec<CubicFace3>,
    in_front: Option<Box<BSPNode>>,
    behind: Option<Box<BSPNode>>,
    /// List of faces that needs to be processed by this node
    /// This is only used during construction of the tree and should
    /// not be used at any other time.
    to_process: Vec<CubicFace3>,
}

impl BSPNode {
    fn new() -> Self {
        Self {
            faces: Vec::new(),
            in_front: None,
            behind: None,
            to_process: Vec::new(),
        }
    }

    fn add_face(&mut self, face: CubicFace3) {
        self.faces.push(face);
    }

    fn get_plane(&self) -> &CubicFace3 {
        &self.faces[0]
    }

    fn set_to_process(&mut self, to_process: Vec<CubicFace3>) {
        self.to_process = to_process;
    }

    // Public methods to visit the tree

    pub fn debug(&self, indent: usize) {
        println!(
            "{:indent$}Node from face: {:?}",
            "",
            self.faces[0],
            indent = indent
        );
        if let Some(node) = &self.in_front {
            println!("{:indent$}(in front): ", "", indent = indent);
            node.deref().debug(indent + 2);
        }
        if let Some(node) = &self.behind {
            println!("{:indent$}(behind): ", "", indent = indent);
            node.deref().debug(indent + 2);
        }
    }

    /// Return the number of nodes starting from here
    #[allow(dead_code)]
    fn len(&self) -> usize {
        1 + if let Some(n) = &self.in_front {
            n.deref().len()
        } else {
            0
        } + if let Some(n) = &self.behind {
            n.deref().len()
        } else {
            0
        }
    }

    #[allow(dead_code)]
    fn in_front(&self) -> &Option<Box<BSPNode>> {
        &self.in_front
    }

    #[allow(dead_code)]
    fn behind(&self) -> &Option<Box<BSPNode>> {
        &self.behind
    }
}

/// Implementation of the rendering using the BSP
impl BSPNode {
    fn render(&self, camera: &Camera, drawer: &mut dyn AbstractFrame) {
        let face3d = self.get_plane();
        if face3d.is_visible_from(&camera) {
            let face2d = face3d.projection(camera);
            drawer.draw_one_face(&face2d);
        }
    }

    pub fn painter_algorithm_traversal(&self, camera: &Camera, drawer: &mut dyn AbstractFrame) {
        // TODO handle collinear faces
        if point_in_front_of(self.get_plane(), camera.pose().position()) {
            // draw in the following order: behind, current, in-fronts
            if let Some(face) = &self.behind {
                face.painter_algorithm_traversal(camera, drawer);
            }
            self.render(camera, drawer);
            if let Some(face) = &self.in_front {
                face.painter_algorithm_traversal(camera, drawer);
            }
        } else {
            // draw in the following order: in-fronts, current, behind
            if let Some(face) = &self.in_front {
                face.painter_algorithm_traversal(camera, drawer);
            }
            self.render(camera, drawer);
            if let Some(face) = &self.behind {
                face.painter_algorithm_traversal(camera, drawer);
            }
        }
    }
}

/// Builds a binary space partitioning of the provided list of polygons.
pub fn binary_space_partionning(faces: &Vec<CubicFace3>) -> BSPNode {
    /// Recursive function in charge of building the BSP.
    /// The function uses the attributes `to_process` as the list of faces that this node
    /// as to classify.
    fn recursive_construction(node: &mut BSPNode) {
        // Select the first face in the list as the main face of the node
        // This is an arbitrary decision.
        node.add_face(node.to_process[0].clone());

        // Split all the other polygon in the list so that they are either stricly in_front or behind
        // the current frame.
        let mut in_fronts = vec![];
        let mut behinds = vec![];
        for i in 1..node.to_process.len() {
            let f = node.to_process[i].clone();
            match bsp_polygon_split(&f, node.get_plane()) {
                (Some(in_front), None) => in_fronts.push(in_front),
                (None, Some(behind)) => behinds.push(behind),
                (Some(in_front), Some(behind)) => {
                    in_fronts.push(in_front);
                    behinds.push(behind);
                }
                (_, _) => {
                    panic!("Not supported : the face necessarly belongs to the remaining space.")
                }
            }
        }

        // Apply this algorithm to the two newly constructed list
        if in_fronts.len() > 0 {
            let mut new_node = BSPNode::new();
            new_node.set_to_process(in_fronts);
            recursive_construction(&mut new_node);
            node.in_front = Some(Box::new(new_node));
        }

        if behinds.len() > 0 {
            let mut new_node = BSPNode::new();
            new_node.set_to_process(behinds);
            recursive_construction(&mut new_node);
            node.behind = Some(Box::new(new_node));
        }
    }

    let mut root = BSPNode::new();
    root.set_to_process(faces.clone());
    recursive_construction(&mut root);
    return root;
}

#[cfg(test)]
mod tests {
    use crate::bsp::cubic_face_split::point_in_front_of;
    use crate::bsp::tree::binary_space_partionning;
    use crate::drawable::Drawable;
    use crate::frame::AbstractFrame;
    use crate::primitives::camera::Camera;
    use crate::primitives::cube::Cube3;
    use crate::primitives::cubic_face2::CubicFace2;
    use crate::primitives::cubic_face3::CubicFace3;
    use crate::primitives::point::Point2;
    use crate::primitives::textures::colored::YELLOW;
    use crate::primitives::vector::{UNIT_Z, Vector3};
    use crate::worlds::World;
    use std::f32::consts::PI;
    use std::time::Instant;

    #[test]
    fn test_bsp_construction1() {
        //     x going down
        //     y going right: -1    0    1
        //
        //                    G----------H          x=-0.5
        //                          A               x=0
        //          P         C     │    D          x=0.5
        //                          B               x=1
        //                    E          F          y=1.5
        //
        let (a, b, c, _d, e, _f, g, h, p) = crate::bsp::tests::get_map();

        let face_ab = CubicFace3::vface_from_line(a, b);
        let face_gh = CubicFace3::vface_from_line(g, h);
        let face_cp = CubicFace3::vface_from_line(c, p);
        let face_ce = CubicFace3::vface_from_line(c, e);

        assert!(point_in_front_of(&face_ab, &c));
        assert!(!point_in_front_of(&face_ab, &h));

        let bsp = binary_space_partionning(&vec![face_ab.clone(), face_gh.clone()]);
        bsp.debug(0);
        assert_eq!(3, bsp.len());
        assert_eq!(1, bsp.in_front().as_ref().unwrap().len());
        assert_eq!(1, bsp.behind().as_ref().unwrap().len());

        let bsp = binary_space_partionning(&vec![
            face_ab.clone(),
            face_gh.clone(),
            face_cp.clone(),
            face_ce.clone(),
        ]);
        // bsp.debug(0);
        assert_eq!(5, bsp.len());
        assert_eq!(3, bsp.in_front().as_ref().unwrap().len());
        assert_eq!(1, bsp.behind().as_ref().unwrap().len());
    }

    struct DummyFrame {
        faces: Vec<[Point2; 4]>,
    }

    impl DummyFrame {
        pub fn new() -> Self {
            Self { faces: vec![] }
        }

        pub fn has_face(&self, f: &CubicFace2) -> bool {
            for face in &self.faces {
                if f.points() == *face {
                    return true;
                }
            }
            false
        }
    }

    impl AbstractFrame for DummyFrame {
        fn draw_one_face(&mut self, face: &CubicFace2) {
            self.faces.push(face.points());
        }
    }

    #[test]
    fn test_bsp_rendering1() {
        let mut world = World::new(Camera::default());
        let f1 = CubicFace3::vface_from_line(Vector3::newi2(0, 0), Vector3::newi2(1, 0));
        let f2 = CubicFace3::vface_from_line(Vector3::newi2(1, 1), Vector3::newi2(2, 1));
        world.add_face(f1.clone());
        world.add_face(f2.clone());

        // Sets the camera as looking at the object
        world.set_camera_position(Vector3::newi2(3, -4));
        world.set_camera_rotation(-PI / 2.);
        world.compute_bsp();

        // Test using the dummy drawer
        let mut drawer = DummyFrame::new();
        world.draw_painter(&mut drawer);

        // Compute the projection
        let f1_p = f1.projection(world.camera());
        let f2_p = f2.projection(world.camera());

        // Assert that an object is visible
        assert!(drawer.has_face(&f1_p));
        assert!(drawer.has_face(&f2_p));
    }

    #[test]
    fn test_bsp_rendering2() {
        let mut world = World::new(Camera::default());
        let f1 = CubicFace3::vface_from_line(Vector3::newi2(0, 0), Vector3::newi2(1, 0));
        let f2 = CubicFace3::vface_from_line(Vector3::newi2(2, 0), Vector3::newi2(3, 0));
        world.add_face(f1.clone());
        world.add_face(f2.clone());

        // Sets the camera as looking at the object
        world.set_camera_position(Vector3::newi2(3, -4));
        world.set_camera_rotation(-PI / 2.);
        world.compute_bsp();
        world.bsp().as_ref().unwrap().debug(0);

        // Test using the dummy drawer
        let mut drawer = DummyFrame::new();
        world.draw_painter(&mut drawer);

        // Compute the projection
        let f1_p = f1.projection(world.camera());
        let f2_p = f2.projection(world.camera());

        // Assert that an object is visible
        assert!(drawer.has_face(&f1_p));
        assert!(drawer.has_face(&f2_p));
    }

    #[test]
    fn bsp_benchmarking_with_dummy() {
        let mut world = World::new(Camera::default());
        world.set_camera_position(Vector3::new(0.11243102, -30.725393, -8.0802684));
        world.set_camera_rotation(-PI / 2.);

        // Create many cubes arranged as a sort of maze
        let n = 6;
        for i in -n..n {
            for j in -n..n {
                let bottom_face = CubicFace3::hface_from_line(
                    Vector3::new(3. * i as f32, 3. * j as f32, 0.0),
                    Vector3::new(3. * i as f32 + 1.0, 3. * j as f32, 0.0),
                );
                let cube = Cube3::from_face(bottom_face, 2.0, &YELLOW);
                world.add_cube(cube);
            }
        }
        // First use standard painter algorithm
        let t0 = Instant::now();
        let mut drawer = DummyFrame::new();
        world.draw_painter(&mut drawer);
        let dt_without = t0.elapsed();
        println!("NO-bsp duration = {:?}", dt_without);

        // Activate BSP
        world.compute_bsp();
        println!("BSP length: {}", world.bsp().as_ref().unwrap().len());

        // Record with BSP
        let t0 = Instant::now();
        let mut drawer = DummyFrame::new();
        world.draw_painter(&mut drawer);
        let dt_with = t0.elapsed();
        println!("BSP duration = {:?}", dt_with);

        assert!(dt_with < dt_without);
    }

    #[test]
    fn bsp_test_minecraft_world() {
        let mut world = World::new(Camera::default());

        // Minecraft blocks
        world.add_cube(Cube3::minecraft_like(Vector3::newi(0,0,0), &YELLOW, &YELLOW));

        // Set the floor
        let x0  = -5;
        let y0  = -5;
        let x1 = 5;
        let y1 = 5;
        world.add_face(CubicFace3::new(
            [Vector3::newi(x0, y0, 1),Vector3::newi(x1, y0, 1),Vector3::newi(x1, y1, 1),Vector3::newi(x0, y1, 1)],
            UNIT_Z.opposite(),
            &YELLOW,
        ));

        world.compute_bsp();
    }
}
