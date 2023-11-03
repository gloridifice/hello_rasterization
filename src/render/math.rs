use nalgebra::{Point3, Rotation3, Unit, Vector3};
use crate::{Mat4, Vec2, Vec2s, Vec3};


pub fn rotation_direction_up(forward: &Vec3, up: &Vec3) -> Mat4{
    Rotation3::face_towards(forward, up).to_homogeneous()
}
pub fn rotation_around_axis(angle: f32, axis: &Vec3) -> Mat4{
    todo!()
}
pub fn rotation_angle_axis(angles: Vec3) -> Mat4{
    Mat4::new_rotation(angles)
}

pub fn look_at(eye: Vec3, point: Vec3, up: Vec3) -> Mat4{
    let eye = Point3::;
    Mat4::look_at_lh(, self.forward, self.up) * translation(self.position)
}
pub fn translation(vec: Vec3) -> Mat4{
    let mut ret = Mat4::identity();
    ret.m41 = vec.x;
    ret.m42 = vec.y;
    ret.m43 = vec.z;
    ret
}