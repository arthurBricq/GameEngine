mod cubic_face_split;
pub mod tree;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::primitives::cubic_face3::CubicFace3;
    use crate::bsp::cubic_face_split::{bsp_polygon_split, point_in_front_of};
    use crate::bsp::tree::binary_space_partionning;
    use crate::primitives::textures::colored::YELLOW;
    use crate::primitives::vector::Vector3;

    fn is_near(left: Vector3, right: Vector3) -> bool {
        (f32::abs(left.x() - right.x()) < 0.0001)
        && (f32::abs(left.y() - right.y()) < 0.0001)
        && (f32::abs(left.z() - right.z()) < 0.0001)
    }

    fn assert_near(left: Vector3, right: Vector3) {
        println!("{left:?} vs {right:?}");
        assert!(is_near(left, right));
    }

    fn contains_near(points: &[Vector3; 4], right: Vector3) -> bool {
        for point in points {
            if is_near(point.clone(), right) {
                return true;
            }
        }
        false
    }

    pub(crate) fn get_map() -> (Vector3, Vector3, Vector3, Vector3, Vector3, Vector3, Vector3, Vector3, Vector3) {
        //                    G----------H          x=-0.5
        //                          A               x=0
        //          P         C     │    D          x=0.5
        //                          B               x=1
        //                    E          F          y=1.5
        let a = Vector3::newi(0, 0, 0);
        let b = Vector3::newi(1, 0, 0);
        let c = Vector3::new(0.5, -1.0, 0.0);
        let d = Vector3::new(0.5, 1.0, 0.0);
        let e = Vector3::new(1.5, -1.0, 0.0);
        let f = Vector3::new(1.5, 1.0, 0.0);
        let g = Vector3::new(-0.5, -1.0, 0.0);
        let h = Vector3::new(-0.5, 1.0, 0.0);
        let p = Vector3::new(0.5, -2.0, 0.0);
        return (a, b, c, d, e, f, g, h, p);
    }

    #[test]
    fn test_point_in_front_of() {
        // f's normal is (1,0,0)
        let f = CubicFace3::vface_from_line(Vector3::newi(0, 0, 0), Vector3::newi(1, 0, 0));
        println!("normal = {:?}", f.normal());

        assert!(!point_in_front_of(&f, &Vector3::newi(0, 1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(1, 1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(2, 2, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(3, 3, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(4, 4, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-1, 1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-2, 2, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-3, 3, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-4, 4, 0)));

        assert!(point_in_front_of(&f, &Vector3::newi(0, -1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(1, -1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(2, -2, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(3, -3, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(4, -4, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-1, -1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-2, -2, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-3, -3, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-4, -4, 0)));
    }

    #[test]
    fn test_line_intersection_with_plane() {
        let (a, b, c, d, e, f, g, h, p) = get_map();
        let face = CubicFace3::vface_from_line(a, b);

        assert_near(face.line_intersection(&c, &d).unwrap(), Vector3::new(0.5, 0.0, 0.0));

        assert_near(face.line_intersection(&e, &f).unwrap(), Vector3::new(1.5, 0.0, 0.0));
        assert_near(face.line_intersection(&f, &e).unwrap(), Vector3::new(1.5, 0.0, 0.0));
        assert_near(face.line_intersection(&g, &h).unwrap(), Vector3::new(-0.5, 0.0, 0.0));

        // Some diagonal intersection
        assert_near(face.line_intersection(&e, &h).unwrap(), Vector3::new(0.5, 0.0, 0.0));
        assert_near(face.line_intersection(&g, &f).unwrap(), Vector3::new(0.5, 0.0, 0.0));

        // Some intersection not balanced around zero
        let y = Vector3::new(0.5, -10.0, 0.0);
        assert_near(face.line_intersection(&y, &d).unwrap(), Vector3::new(0.5, 0.0, 0.0));

        // Check no intersection
        assert_eq!(face.line_intersection(&c, &e), None);
        assert_eq!(face.line_intersection(&c, &g), None);
        assert_eq!(face.line_intersection(&d, &f), None);
    }

    #[test]
    /// Test that the polygon splitting algorithm works
    fn test_bsp_polygon_splitting() {
        let (a, b, c, d, e, f, g, h, p) = get_map();

        let face_ab = CubicFace3::vface_from_line(a, b);
        let face_gh = CubicFace3::vface_from_line(g, h);
        let face_gc = CubicFace3::vface_from_line(g, c);
        let face_hf = CubicFace3::vface_from_line(h, f);
        let face_cp = CubicFace3::vface_from_line(c, p);

        // The face GH must be split in two faces
        let (r1, r2) = bsp_polygon_split(&face_gh, &face_ab);
        assert!(r1.is_some());
        assert!(r2.is_some());
        let f1 = r1.unwrap();
        let f2 = r2.unwrap();
        assert!(f1.points().contains(&Vector3::new(-0.5, 0.0, 0.0)));
        assert!(f1.points().contains(&Vector3::new(-0.5, 0.0, 2.0)));
        assert!(f2.points().contains(&Vector3::new(-0.5, 0.0, 0.0)));
        assert!(f2.points().contains(&Vector3::new(-0.5, 0.0, 2.0)));

        // The gc face is behind
        let (r1, r2) = bsp_polygon_split(&face_gc, &face_ab);
        assert!(r2.is_none());
        assert!(r1.is_some());

        // Same for CP
        let (r1, r2) = bsp_polygon_split(&face_cp, &face_ab);
        assert!(r2.is_none());
        assert!(r1.is_some());

        // The gc face is in front
        let (r1, r2) = bsp_polygon_split(&face_hf, &face_ab);
        assert!(r2.is_some());
        assert!(r1.is_none());

        // between h and e, there should be an intersection
        let face_eh = CubicFace3::vface_from_line(e, h);
        let (r1, r2) = bsp_polygon_split(&face_eh, &face_ab);
        assert!(r2.is_some());
        assert!(r1.is_some());
        let f1 = r1.unwrap();
        let f2 = r2.unwrap();
        println!("f1 = {f1:?}");
        println!("f2 = {f2:?}");
        // We must assert that contains 'near' works
        assert!(contains_near(&f2.points(), Vector3::new(0.5, 0.0, 0.0)));
        assert!(contains_near(&f2.points(), Vector3::new(0.5, 0.0, 2.0)));
        assert!(contains_near(&f1.points(), Vector3::new(0.5, 0.0, 0.0)));
        assert!(contains_near(&f1.points(), Vector3::new(0.5, 0.0, 2.0)));
    }

    #[test]
    fn test_bsp_polygon_splitting_face_partially_included_in_splitter() {
        let to_split = CubicFace3::new(
            [Vector3::newi2(0, 0), Vector3::newi2(2, 0), Vector3::newi2(2, 2), Vector3::newi2(0, 2)],
            Vector3::newi(0,0,-1),
            &YELLOW
        );
        let splitter = CubicFace3::new(
            [ Vector3::newi(0, 0, 0), Vector3::newi(0, 1, 0), Vector3::newi(0, 1, 1), Vector3::newi(0, 0, 1)],
            Vector3::newi(-1, 0 ,0),
            &YELLOW
        );

        let (r1, r2) = bsp_polygon_split(&to_split, &splitter);
        println!("{r1:?}");
        println!("{r2:?}");
    }

    #[test]
    fn test_bsp_polygon_splitting3() {
        let to_split = CubicFace3::new(
            [Vector3::newi2(0,0), Vector3::newi2(2, 0), Vector3::newi2(2, 2), Vector3::newi2(0, 2)],
            Vector3::newi(0,0,-1),
            &YELLOW
        );
        let splitter = CubicFace3::new(
            [Vector3::newi2(0, 1), Vector3::newi2(2, 1), Vector3::newi(2, 1, -1), Vector3::newi(0, 1, -1)],
            Vector3::newi(0, -1, 0),
            &YELLOW
        );
        assert_eq!(4., to_split.area());
        assert_eq!(2., splitter.area());

        let (r1, r2) = bsp_polygon_split(&to_split, &splitter);
        let f1 = r1.unwrap();
        let f2 = r2.unwrap();
        assert_eq!(4., f1.area() + f2.area());
        assert!(f1.area() < 4.);
        assert!(f2.area() < 4.);
    }
}
