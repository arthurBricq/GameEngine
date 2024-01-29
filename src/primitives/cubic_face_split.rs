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
pub fn spit_in_front_and_behind(to_split: &CubicFace3, face: &CubicFace3) -> (Option<CubicFace3>, Option<CubicFace3>) {
    // The algo is very simple : since the polygon are convex and have 4 points, we can diffenriate 3 scenarios
    // * 1: all the points of `to_split` are in front of `face`
    // * 2: all the points of `to_split` are behind `face`
    // * 3: two points are behind, two points are front

    // Note that this algorithm is a super-simplified version of polygon splitting algorithm,
    // which works in my case.

    // Compute the number of points in front of the face
    let in_front = to_split.points().iter().filter(|p| point_in_front_of(&face, &p)).count();

    match in_front {
        0 => {
            // all points behind
            return (None, Some(*face.clone()))
        },
        2 => {},
        4 => {},
        _ => {panic!("Unsupported number of points in front of the face: {in_front}")}
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
    use crate::primitives::cubic_face_split::point_in_front_of;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_point_in_front_of(){
        // f's normal is (1,0,0)
        let f = CubicFace3::vface_from_line(Vector3::newi(0,0,0), Vector3::newi(1,0,0));
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
        let a = Vector3::newi(0,0,0);
        let b = Vector3::newi(1,0,0);
        let c = Vector3::new(0.5, -1.0, 0.0);
        let d = Vector3::new(0.5, 1.0, 0.0);
        let e = Vector3::new(1.5, -1.0, 0.0);
        let f = Vector3::new(1.5, 1.0, 0.0);
        let g = Vector3::new(-0.5, -1.0, 0.0);
        let h = Vector3::new(-0.5, 1.0, 0.0);

        let face = CubicFace3::vface_from_line(a, b);

        assert_eq!(face.line_intersection(&c, &d), Some( Vector3::new(0.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&e, &f), Some( Vector3::new(1.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&f, &e), Some( Vector3::new(1.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&g, &h), Some( Vector3::new(-0.5, 0.0, 0.0)));

        // Some diagonal intersection
        assert_eq!(face.line_intersection(&e, &h), Some( Vector3::new(0.5, 0.0, 0.0)));
        assert_eq!(face.line_intersection(&g, &f), Some( Vector3::new(0.5, 0.0, 0.0)));

        // Some intersection not balanced around zero
        let y = Vector3::new(0.5, -10.0, 0.0);
        assert_eq!(face.line_intersection(&y, &d), Some( Vector3::new(0.5, 0.0, 0.0)));

        // Some lines that do not intersect
        assert_eq!(face.line_intersection(&y, &c), None);
        assert_eq!(face.line_intersection(&c, &y), None);
        assert_eq!(face.line_intersection(&c, &e), None);
        assert_eq!(face.line_intersection(&c, &g), None);
        assert_eq!(face.line_intersection(&d, &f), None);
    }

    #[test]
    fn bsp_polygon_splitting() {

    }
}
