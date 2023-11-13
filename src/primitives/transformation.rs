/// Represent an homogenous transformation of the 3D space to the 3D space
pub struct Transform {
    tx: f32,
    ty: f32,
    tz: f32,
    // how to represent the rotation ?
    a11: f32,
    a12: f32,
    a13: f32,
    a21: f32,
    a22: f32,
    a23: f32,
    a31: f32,
    a32: f32,
    a33: f32,
}