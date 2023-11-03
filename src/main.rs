use anyhow::Result;
use colored::*;
use hello_graphics::app::App;
use hello_graphics::io::color::Color;
use hello_graphics::io::image::Image;
use hello_graphics::io::model::Model;
use hello_graphics::render::{line, rasterization, warframe_model};

fn main() -> Result<()> {
    let mut image = Image::new(512, 512);

    image.clear(Color::white());
    let model = Model::new_load("resources/obj/african_head.obj")?;
    // warframe_model(&mut image, &model, Color::black())?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "hello graphics",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    ).unwrap();

    Ok(())
}

#[cfg(test)]
mod test{

}