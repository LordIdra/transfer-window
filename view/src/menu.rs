use eframe::egui::{CentralPanel, Context};

use crate::events::Event;

#[derive(Default)]
pub struct Scene {}

impl Scene {
    pub fn update(&mut self, context: &Context) -> Vec<Event> {
        let mut events = vec![];
        CentralPanel::default().show(context, |ui| {
            ui.label("The best menu ever created");
            if ui.button("New game").clicked() {
                events.push(Event::NewGame);
            }
            if ui.button("Load game").clicked() {
                events.push(Event::LoadGame { name: "test".to_owned() });
            }
            if ui.button("Quit").clicked() {
                events.push(Event::Quit);
            }
        });
        events
    }
}