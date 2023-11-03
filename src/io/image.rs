use std::fmt::format;
use crate::io::color::Color;
use anyhow::Result;
use png::{Encoder, Writer};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use colored::{ColoredString, Colorize};
use egui::Ui;
use nalgebra::{partial_sort2, point};
use obj::{load_obj, Obj};
use crate::{Vec2i, Vec2s};


pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
    texture: Option<egui::TextureHandle>,
}


impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color::with_rgb(0, 0, 0); (width * height) as usize],
            texture: None,
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        let color = self.as_u8_array();
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                egui::ColorImage::from_rgba_unmultiplied([self.width as usize, self.height as usize], color.as_slice()),
                Default::default(),
            )
        });

        // Show the image:
        ui.image(texture, texture.size_vec2());
    }
    pub fn as_u8_array(&self) -> Vec<u8> {
        let size = self.width * self.height * 4;
        let mut colors = vec![0u8; size as usize];

        for i in 0..self.pixels.len() {
            let color = &self.pixels[i].as_u8();

            colors[i * 4] = color[0];
            colors[i * 4 + 1] = color[1];
            colors[i * 4 + 2] = color[2];
            colors[i * 4 + 3] = color[3];
        }
        colors
    }
    pub fn set_pixel_color(&mut self, pos: Vec2s, color: Color) -> Result<()> {
        let (x, y) = (pos.x, pos.y);
        if x < 0 || x >= self.width || y < 0 || y >= self.height{ return Ok(()); }
        let index = (x + y * self.width) as usize;

        self.pixels[index] = color;

        Ok(())
    }
    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            pixel.set_to(color);
        }
    }
    fn colors_to_u8_vec(&self) -> Vec<u8> {
        let mut ret = Vec::with_capacity(self.pixels.len() * 4);
        for x in &self.pixels {
            ret.extend_from_slice(&x.as_u8());
        }
        ret
    }
    pub fn write_to_png(self, path: &str) {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(self.colors_to_u8_vec().as_ref()).unwrap();
        writer.finish().expect("png Drawing Finished Failed.");
    }
}
