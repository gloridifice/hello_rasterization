use std::time::SystemTime;
use nalgebra::{max, min};
use crate::io::color::Color;
use crate::io::image::Image;
use crate::{Vec2i, Vec2s, Vec3};
use crate::io::model::Model;
use anyhow::Result;

pub fn inside_triangle(tri: (Vec3, Vec3, Vec3), point: Vec3) -> bool {
    let is_left = |line_pos0: Vec3, line_pos1: Vec3, point: Vec3| -> bool {
        (line_pos1.x - line_pos0.x) * (point.y - line_pos0.y) - (line_pos1.y - line_pos0.y) * (point.x - line_pos0.x) >= 0.
    };
    is_left(tri.0, tri.1, point) && is_left(tri.1, tri.2, point) && is_left(tri.2, tri.0, point)
}

pub fn triangle(color_buffer: &mut Image, z_buffer: &mut Image, tri: (Vec3, Vec3, Vec3), normal: Vec3, color: Color) {
    let (x_min, x_max, y_min, y_max) = (
        tri.0.x.min(tri.1.x.min(tri.2.x)) as i32,
        tri.0.x.max(tri.1.x.max(tri.2.x)) as i32,
        tri.0.y.min(tri.1.y.min(tri.2.y)) as i32,
        tri.0.y.max(tri.1.y.max(tri.2.y)) as i32 );
    let light: Vec3 = Vec3::new(0., -1., -1.).normalize();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let pos = Vec3::new(x as f32, y as f32, 0.);
            if inside_triangle(tri, pos) {
                let light_color = (normal.dot(&light).cos() + 1.) / 2.;
                color_buffer.set_pixel_color(Vec2s::new(x as usize, y as usize), color * light_color).unwrap();
            }
        }
    }
}

pub fn rasterize(color_buffer: &mut Image, z_buffer: &mut Image, model: &Model, color: Color) -> Result<()> {
    let time = SystemTime::now();
    let vertices = model.vertices()?;
    for face in model.faces().unwrap() {
        let (point0, point1, point2) = (vertices[face.vertices_indices[0]], vertices[face.vertices_indices[1]], vertices[face.vertices_indices[2]]);
        triangle(color_buffer, z_buffer, (point0.position, point1.position, point2.position), face.normal, color);
    }

    println!("Rendering model use time: {}s", time.elapsed().unwrap().as_millis() as f64 / 1000.);
    Ok(())
}
