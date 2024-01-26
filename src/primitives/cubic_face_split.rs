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
pub fn spit_in_front_and_behind(to_be_splitted: &CubicFace3, face: &CubicFace3) -> (Option<CubicFace3>, Option<CubicFace3>) {
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
    fn test_line_intersection_with_place() {
        let f = CubicFace3::vface_from_line(Vector3::newi(0,0,0), Vector3::newi(1,0,0));
        let tmp = f.line_intersection(&Vector3::new(0.5, -1.0, 0.0), &Vector3::new(0.5, 1.0, 0.0));
        assert!(tmp.is_some());
        assert_eq!(tmp.unwrap(), Vector3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn bsp_polygon_splitting() {}
}
