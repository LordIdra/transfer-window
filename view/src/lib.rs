use std::sync::Arc;

use eframe::{egui::{CentralPanel, Context}, glow};
use transfer_window_model::Model;

pub struct View {
    gl: Arc<glow::Context>,
}

impl View {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        Self { gl }
    }

    pub fn update(&self, context: &Context, model: &Model) {
        CentralPanel::default().show(context, |ui| {
            ui.label("Hello Terence");
        });
    }
}