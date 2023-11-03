use std::fs::File;
use std::io::BufReader;
use std::process::Termination;
use std::time::SystemTime;
use obj::{load_obj, Obj};
use crate::io::color::Color;
use crate::io::image::Image;
use crate::{Mat4, Vec2i, Vec2s, Vec3};
use anyhow::Result;
use eframe::get_value;
use nalgebra::{Matrix, Matrix3, Matrix4};
use crate::io::model::Model;

pub mod shader;
pub mod camera;
pub mod rasterization;
mod math;
mod texture_2d;


pub fn line(image: &mut Image, pos0: Vec2i, pos1: Vec2i, color: Color) -> Result<()> {
    let mut steep = false;
    let (mut x0, mut y0, mut x1, mut y1) = if (pos0.x - pos1.x).abs() < (pos0.y - pos1.y).abs() {
        steep = true;
        (pos0.y, pos0.x, pos1.y, pos1.x)
    } else { (pos0.x, pos0.y, pos1.x, pos1.y) };

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let (dx, dy) = (x1 - x0, y1 - y0);
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            image.set_pixel_color(Vec2s::new(y as usize, x as usize), color)?;
        } else {
            image.set_pixel_color(Vec2s::new(x as usize, y as usize), color)?;
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
    Ok(())
}

pub fn warframe_model(image: &mut Image, model: &Model, color: Color) -> Result<()> {
    let time = SystemTime::now();
    let vertices = model.vertices()?;
    for face in model.faces().unwrap() {
        let (point0, point1, point2) = (vertices[face.vertices_indices[0]], vertices[face.vertices_indices[1]], vertices[face.vertices_indices[2]]);

        let tran_pos = |pos: Vec3| -> Vec2i {
            Vec2i::new(((pos[0] + 1.) * image.width as f32 / 2.).floor() as i32, ((pos[1] + 1.) * image.height as f32 / 2.).floor() as i32)
        };
        let p0: Vec2i = tran_pos(point0.position);
        let p1: Vec2i = tran_pos(point1.position);
        let p2: Vec2i = tran_pos(point2.position);
        line(image, p0, p1, color)?;
        line(image, p1, p2, color)?;
        line(image, p2, p0, color)?;
    }

    println!("Rendering model use time: {}s", time.elapsed().unwrap().as_millis() as f64 / 1000.);
    Ok(())
}


