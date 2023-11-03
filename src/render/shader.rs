use anyhow::Result;
use crate::io::color::Color;
use crate::Vec3;

pub struct Uniform{
    pub main_light_dir: Vec3
}
pub struct VertexAttribute{
    pub position_os: Vec3,
    pub normal_os: Vec3
}
pub type VertexShader = fn(attribute: VertexAttribute, uniform: Uniform);

pub struct FragAttribute{
    pub position_os: Vec3,
    pub normal_os: Vec3,
}
pub type FragShader = fn(attribute: FragAttribute, uniform: Uniform) -> Result<Color>;