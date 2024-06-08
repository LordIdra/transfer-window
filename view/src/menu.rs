use eframe::egui::{CentralPanel, Context};

use crate::controller_events::ControllerEvent;

#[derive(Default)]
pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, context: &Context) -> Vec<ControllerEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");

        let mut events = vec![];
        CentralPanel::default().show(context, |ui| {
            ui.label("The best menu ever created");
            if ui.button("New game").clicked() {
                events.push(ControllerEvent::NewGame);
            }
            if ui.button("Load game").clicked() {
                events.push(ControllerEvent::LoadGame { name: "test".to_owned() });
            }
            if ui.button("Quit").clicked() {
                events.push(ControllerEvent::Quit);
            }
        });
        events
    }
}