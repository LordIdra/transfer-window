use eframe::{egui::{Align2, Color32, RichText, Ui, Window}, epaint};

use crate::game::View;

use super::widgets::custom_image::CustomImage;

const FADE_OUT_TIME: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct Objective {
    objective: &'static str,
    complete: bool,
    opacity: f32,
}

impl Objective {
    pub fn new(objective: &'static str) -> Self {
        let complete = false;
        let opacity = 1.0;
        Self { objective, complete, opacity }
    }

    pub fn update(&mut self, dt: f64) {
        if self.complete {
            self.opacity -= dt as f32 / FADE_OUT_TIME;
            self.opacity = f32::max(0.0, self.opacity);
        }
    }

    pub fn finished(&self) -> bool {
        self.opacity == 0.0
    }

    pub fn set_complete(&mut self) {
        self.complete = true;
    }
    
    pub fn objective(&self) -> &str {
        self.objective
    }

    pub fn draw(&self, view: &View, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let texture = match self.complete {
                false => "objective-incomplete",
                true => "objective-complete",
            };
            ui.add(CustomImage::new(view, texture, 12.0).with_alpha(self.opacity));
            ui.label(RichText::new(self.objective)
                .monospace()
                .color(Color32::from_rgba_unmultiplied(255, 255, 255, (self.opacity * 255.0) as u8)));
        });
    }
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update objectives");

    Window::new("Objectives")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, epaint::vec2(0.0, 50.0))
        .show(&view.context.clone(), |ui| {
            for objective in &view.objectives {
                objective.draw(view, ui);
            }
        });
}