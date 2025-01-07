use eframe::egui::{CentralPanel};

#[derive(Default)]
pub struct Simulation;

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Test!");
            if ui.button("this is a button").clicked() {
                println!("this is a test");
            }
        });
    }
}