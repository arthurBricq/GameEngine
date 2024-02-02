use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::vector::Vector3;

/// Helper function for the binary space partitioning.
///
/// Splits a given polygon into possibly 1 or 2 parts that are strictly
/// in front another, or behind it.
///
/// Resources
/// * https://geidav.wordpress.com/2015/03/21/splitting-an-arbitrary-polygon-by-a-line/
/// * https://stackoverflow.com/questions/3623703/how-can-i-split-a-polygon-by-a-line
/// * Chapter VII.4 of the book "Graphics Gems 5"
/// * From the book "Graphics Gems 3", the chapter:
/// "PARTITIONING A 3-D CONVEXARTITIONING A 3-D CONVEXARTITIONING A 3-D CONVEXARTITIONING A 3-D CONVEXARTITIONING A 3-D CONVEX"
///
pub fn bsp_polygon_split(to_split: &CubicFace3, face: &CubicFace3) -> (Option<CubicFace3>, Option<CubicFace3>) {
    // The algo is very simple : since the polygon are convex and have 4 points, we can diffenriate 3 scenarios
    // * 1: all the points of `to_split` are in front of `face`
    // * 2: all the points of `to_split` are behind `face`
    // * 3: two points are behind, two points are front
    //   For this configuration, we only support two modes: horizontal split or vertical split.

    // Note that this algorithm is a super-simplified version of polygon splitting algorithm,
    // which works in my case.

    // Compute the number of points in front of the face
    let points = to_split.points();
    let n_in_front = points.iter().filter(|p| point_in_front_of(&face, &p)).count();

    match n_in_front {
        // all points behind
        0 => return (None, Some(face.clone())),
        // two points are in front, two points are behind: we need to
        // split the polygon in two.
        2 => {
            // Compute the points in front
            let in_fronts: Vec<bool> = to_split.points().iter().map(|p| point_in_front_of(&face, &p)).collect();
            enum SplitMode { AfterFirst, AfterSecond }
            let mut split_mode = if in_fronts[0] != in_fronts[1] {
                SplitMode::AfterFirst
            } else {
                SplitMode::AfterSecond
            };

            match split_mode {
                SplitMode::AfterFirst => {
                    let x = face.line_intersection(&points[0], &points[1]).unwrap();
                    let y = face.line_intersection(&points[2], &points[3]).unwrap();
                    let f1 = CubicFace3::new([points[0], x, y, points[3]], to_split.normal().clone(), to_split.texture().clone());
                    let f2 = CubicFace3::new([x, points[1], points[2], y], to_split.normal().clone(), to_split.texture().clone());
                    return (Some(f1), Some(f2))
                }
                SplitMode::AfterSecond => {
                    let x = face.line_intersection(&points[1], &points[2]).unwrap();
                    let y = face.line_intersection(&points[3], &points[0]).unwrap();
                    let f1 = CubicFace3::new([points[0], points[1], x, y], to_split.normal().clone(), to_split.texture().clone());
                    let f2 = CubicFace3::new([y, x, points[2], points[3]], to_split.normal().clone(), to_split.texture().clone());
                    return (Some(f1), Some(f2))

                }
            }
        }
        // all the points are in front
        4 => return (Some(face.clone()), None),
        _ => { panic!("Unsupported number of points in front of the face: {n_in_front}") }
    }

    return (None, None);
}

/// Returns true if the given point is in front of the plane, false otherwise.
fn point_in_front_of(face: &CubicFace3, point: &Vector3) -> bool {
    let to_center = point.line_to(&face.center());
    to_center.dot(face.normal()) > 0.0
}


#[cfg(test)]
mod tests {
    use crate::primitives::cubic_face3::CubicFace3;
    use crate::primitives::cubic_face_split::{bsp_polygon_split, point_in_front_of};
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_point_in_front_of() {
        // f's normal is (1,0,0)
        let f = CubicFace3::vface_from_line(Vector3::newi(0, 0, 0), Vector3::newi(1, 0, 0));
        println!("normal = {:?}", f.normal());

        assert!(point_in_front_of(&f, &Vector3::newi(0, 1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(1, 1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(2, 2, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(3, 3, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(4, 4, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-1, 1, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-2, 2, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-3, 3, 0)));
        assert!(point_in_front_of(&f, &Vector3::newi(-4, 4, 0)));

        assert!(!point_in_front_of(&f, &Vector3::newi(0, -1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(1, -1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(2, -2, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(3, -3, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(4, -4, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-1, -1, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-2, -2, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-3, -3, 0)));
        assert!(!point_in_front_of(&f, &Vector3::newi(-4, -4, 0)));
    }

    #[test]
    ///                    G          H
    ///
    ///                          A
    ///                          │
    ///                          │
    ///                          │
    ///          P         C     │    D
    ///                          │
    ///                          │
    ///                          │
    ///                          B
    ///
    ///                    E           F
    fn test_line_intersection_with_plane() {
        let a = Vector3::newi(0, 0, 0);
        let b = Vector3::newi(1, 0, 0);
        let c = Vector3::new(0.5, -1.0, 0.0);
        let d = Vector3::new(0.5, 1.0, 0.0);
        let e = Vector3::new(1.5, -1.0, 0.0);
        let f = Vector3::new(1.5, 1.0, 0.0);
        let g = Vector3::new(-0.5, -1.0, 0.0);
        let h = Vector3::new(-0.5, 1.0, 0.0);
        let face = CubicFace3::vface_from_line(a, b);

        assert_eq!(face.line_intersection(&c, &d), Some(Vector3::new(0.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&e, &f), Some(Vector3::new(1.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&f, &e), Some(Vector3::new(1.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&g, &h), Some(Vector3::new(-0.5, 0.0, 0.0)));

        // Some diagonal intersection
        assert_eq!(face.line_intersection(&e, &h), Some(Vector3::new(0.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&g, &f), Some(Vector3::new(0.5, 0.0, 0.0)));

        // Some intersection not balanced around zero
        let y = Vector3::new(0.5, -10.0, 0.0);
        assert_eq!(face.line_intersection(&y, &d), Some(Vector3::new(0.5, 0.0, 0.0)));

        // Some lines that do not intersect
        assert_eq!(face.line_intersection(&y, &c), None);
        assert_eq!(face.line_intersection(&c, &y), None);
        assert_eq!(face.line_intersection(&c, &e), None);
        assert_eq!(face.line_intersection(&c, &g), None);
        assert_eq!(face.line_intersection(&d, &f), None);
    }

    #[test]
    /// Test that the polygon splitting algorithm works
    fn test_bsp_polygon_splitting() {
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
        assert!(r1.is_none());
        assert!(r2.is_some());

        // Same for CP
        let (r1, r2) = bsp_polygon_split(&face_cp, &face_ab);
        assert!(r1.is_none());
        assert!(r2.is_some());

        // The gc face is in front
        let (r1, r2) = bsp_polygon_split(&face_hf, &face_ab);
        assert!(r1.is_some());
        assert!(r2.is_none());

        // between h and e, there should be an intersection
        let face_eh = CubicFace3::vface_from_line(e, h);
        let (r1, r2) = bsp_polygon_split(&face_eh, &face_ab);
        assert!(r1.is_some());
        assert!(r2.is_some());
        let f1 = r1.unwrap();
        let f2 = r2.unwrap();
        assert!(f1.points().contains(&Vector3::new(0.5, 0.0, 0.0)));
        assert!(f1.points().contains(&Vector3::new(0.5, 0.0, 2.0)));
        assert!(f2.points().contains(&Vector3::new(0.5, 0.0, 0.0)));
        assert!(f2.points().contains(&Vector3::new(0.5, 0.0, 2.0)));

    }
}
