use std::iter::Map;
use std::ops::Mul;
use egui::epaint::ahash::HashMap;
use nalgebra::Point3;
use crate::render::camera::ProjectionMode::{Orthogonal, Perspective};
use crate::{Mat4, Vec3};
use crate::io::image::Image;
use crate::render::math::translation;
use crate::render::Texture2D;
use crate::render::texture_2d::{Texture2D, Texture2DBuffer, TextureDimension};

pub enum ProjectionMode {
    Perspective { fov_y: f32 },
    Orthogonal { size_y: f32 },
}

impl ProjectionMode {
    pub fn new_perspective() -> ProjectionMode {
        Perspective { fov_y: 70.}
    }
    pub fn new_orthogonal() -> ProjectionMode {
        Orthogonal { size_y: 10.}
    }
    pub fn fov_y(self, value: f32) -> ProjectionMode {
        if let Perspective {fov_y} = self {
            Perspective {fov_y: value}
        }else { self }
    }
    pub fn size_y(self, value: f32) -> ProjectionMode{
        if let Orthogonal {size_y } = self{
            Orthogonal {size_y: value}
        }else { self }
    }
}

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub width: usize,
    pub height: usize,
    pub near: f32,
    pub far: f32,
    pub projection_mode: ProjectionMode,
    pub output_textures: HashMap<String, Image>
}

impl Camera {
    pub fn render(&mut self){
        let main = Image::new(self.width, self.height);
        
        let projection = self.projection_matrix();
        let view = self.view_matrix();
    }
}
impl Camera {
    pub fn new(width: usize, height: usize, projection: ProjectionMode) -> Camera {
        Camera {
            position: Vec3::zeros(),
            forward: Vec3::new(0., 0., 1.),
            up: Vec3::new(0., 1., 0.),
            width,
            height,
            near: 0.1,
            far: 1000.,
            projection_mode: projection,
            output_textures: HashMap::default(),
        }
    }
    pub fn aspect(&self) -> f32{
        self.width as f32 / self.height as f32
    }
    pub fn projection_matrix(&self) -> Mat4{
        match self.projection_mode {
            Perspective {fov_y} => {
                Mat4::new_perspective(self.aspect(), fov_y, self.near, self.far)
            }
            Orthogonal {size_y} => {

                let corner_offset = Vec3::new(self.aspect() * size_y/2., size_y / 2., 0.);
                let pos_right_top = self.position + corner_offset;
                let pos_left_button = self.position - corner_offset;

                Mat4::new_orthographic(pos_left_button.x, pos_right_top.x, pos_left_button.y, pos_right_top.y, self.near, self.far)
            }
        }
    }
    pub fn look_at_matrix(forward : Vec3, up: Vec3) -> Mat4{

        let zaxis = forward;
        let xaxis = zaxis.cross(&up).normalize();
        let yaxis = zaxis.cross(&xaxis);
        
        todo!()
    }
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_lh(self.position, self.forward, self.up) * translation(self.position)
    }
}