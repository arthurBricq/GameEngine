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
        0 => return (None, Some(to_split.clone())),
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
                    return (Some(f1), Some(f2));
                }
                SplitMode::AfterSecond => {
                    let x = face.line_intersection(&points[1], &points[2]).unwrap();
                    let y = face.line_intersection(&points[3], &points[0]).unwrap();
                    let f1 = CubicFace3::new([points[0], points[1], x, y], to_split.normal().clone(), to_split.texture().clone());
                    let f2 = CubicFace3::new([y, x, points[2], points[3]], to_split.normal().clone(), to_split.texture().clone());
                    return (Some(f1), Some(f2));
                }
            }
        }
        // all the points are in front
        4 => return (Some(to_split.clone()), None),
        _ => { panic!("Unsupported number of points in front of the face: {n_in_front}") }
    }
}

/// Returns true if the given point is in front of the plane, false otherwise.
pub fn point_in_front_of(face: &CubicFace3, point: &Vector3) -> bool {
    let to_center = point.line_to(&face.center());
    to_center.dot(face.normal()) < 0.0
}

