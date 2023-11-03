use std::time::Duration;
use eframe::{Frame, Storage};
use eframe::emath::Vec2;
use egui::{Context, Visuals};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App{
    label: String,

    #[serde(skip)]
    value: f32,
}


impl Default for App {
    fn default() -> Self {
        Self{
            label: "Hello Graphics!".to_owned(),
            value: 2.7,
        }
    }
}

impl App{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self{
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let Self { label, value } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui|{
            ui.heading("Side Panel");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT) ,|ui| {

            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        eframe::set_value(_storage, eframe::APP_KEY, self);
    }
}