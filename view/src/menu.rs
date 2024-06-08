use eframe::egui::{CentralPanel, Context, Key, Window};

use crate::controller_events::ControllerEvent;

#[derive(Default)]
pub struct View {
    debug_window_open: bool,
}

impl View {
    pub fn new() -> Self {
        let debug_window_open = false;
        Self { debug_window_open }
    }

    pub fn update(&mut self, context: &Context) -> Vec<ControllerEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");

        if context.input(|input| input.key_pressed(Key::F12)) {
            self.debug_window_open = !self.debug_window_open;
        }

        let mut events = vec![];

        if self.debug_window_open {
            Window::new("Debug")
                    .show(context, |ui| {
                if ui.button("Load game").clicked() {
                    events.push(ControllerEvent::LoadGame { name: "debug".to_owned() });
                }
            });
        }

        CentralPanel::default().show(context, |ui| {
            ui.label("The best menu ever created");
            if ui.button("New game").clicked() {
                events.push(ControllerEvent::NewGame);
            }
            if ui.button("Quit").clicked() {
                events.push(ControllerEvent::Quit);
            }
        });
        events
    }
}