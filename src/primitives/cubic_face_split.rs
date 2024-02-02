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

            // Find the discontinuities: the links where the points go from in front to behind
            // let mut discontinuities = vec![0usize;2];
            // let mut idx = 0;
            // for i in 1..in_fronts.len() {
            //     if in_fronts[i] != in_fronts[i-1] {
            //         discontinuities[idx] = i;
            //         idx += 1;
            //     }
            // }
            // assert_eq!(idx, 2);
            // let i1 = discontinuities[0];
            // let i2 = discontinuities[1];
            // assert!(i1 < i2);

            // Find the discontinuities: the links where the points go from in front of behind
            let (i1, i2) = match (in_fronts[0] == in_fronts[1], in_fronts[1] == in_fronts[2],
                   in_fronts[2] == in_fronts[3], in_fronts[3] == in_fronts[0]) {
                (true, true, false, false) => (0, 1),
                (true, false, true, false) => (0, 2),
                (true, false, false, true) => (0, 3),
                (false, true, true, false) => (1, 2),
                (false, true, false, true) => (1, 3),
                (false, false, true, true) => (2, 3),
                _ => {panic!("not support pattern")}
            };
            println!("Splitting edges: {i1}, {i2}");

            // Create the resulting faces
            // For now, I am not sure how to handle the case of triangles.
            // Let's think about it later on.
            match (i1, i2) {
                (0, 1) => panic!("triangle not supported"),
                (0, 2) => {
                    let x = face.line_intersection(&points[0], &points[1]).unwrap();
                    let y = face.line_intersection(&points[2], &points[3]).unwrap();
                    let f1 = CubicFace3::new([points[0], x, y, points[3]], to_split.normal().clone(), to_split.texture().clone());
                    let f2 = CubicFace3::new([x, points[1], points[2], y], to_split.normal().clone(), to_split.texture().clone());
                    return (Some(f1), Some(f2))
                }
                (0, 3) => panic!("triangle not supported"),
                (1, 2) => panic!("triangle not supported"),
                (1, 3) => {
                    let x = face.line_intersection(&points[1], &points[2]).unwrap();
                    let y = face.line_intersection(&points[3], &points[0]).unwrap();
                    let f1 = CubicFace3::new([points[0], points[1], x, y], to_split.normal().clone(), to_split.texture().clone());
                    let f2 = CubicFace3::new([y, x, points[2], points[3]], to_split.normal().clone(), to_split.texture().clone());
                    return (Some(f1), Some(f2))
                }
                (2, 3) => panic!("triangle not supported"),
                (_, _) => {panic!("Unsupported set of splitting lines: {i1} and {i2}")}
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
    ///          y         C     │    D
    ///                          │
    ///                          │
    ///                          │
    ///                          B
    ///
    ///                    E           F
    fn test_line_intersection_with_place() {
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
        //                    G----------H
        //                          A
        //                          │
        //          y         C     │    D
        //                          │
        //                          B
        //                    E          F
        let a = Vector3::newi(0, 0, 0);
        let b = Vector3::newi(1, 0, 0);
        let c = Vector3::new(0.5, -1.0, 0.0);
        let d = Vector3::new(0.5, 1.0, 0.0);
        let e = Vector3::new(1.5, -1.0, 0.0);
        let f = Vector3::new(1.5, 1.0, 0.0);
        let g = Vector3::new(-0.5, -1.0, 0.0);
        let h = Vector3::new(-0.5, 1.0, 0.0);

        let face_ab = CubicFace3::vface_from_line(a, b);
        let face_gh = CubicFace3::vface_from_line(g, h);

        let result = bsp_polygon_split(&face_gh, &face_ab);
        println!("result = {result:#?}")

    }
}
