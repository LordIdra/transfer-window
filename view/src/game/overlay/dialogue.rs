use eframe::{egui::{Align2, RichText, Window}, epaint};

use crate::game::{storyteller::story::story_event::StoryEvent, View};

#[derive(Debug, Clone)]
pub struct Dialogue {
    character: &'static str,
    text: &'static str,
}

impl Dialogue {
    pub fn new(character: &'static str, text: &'static str) -> Self {
        Self { character, text }
    }
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update dialogue");

    let Some(dialogue) = view.dialogue.clone() else {
        return;
    };

    Window::new("Explorer")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(200.0, 200.0))
            .show(&view.context.clone(), |ui| {
        ui.label(RichText::new(dialogue.character).strong());
        ui.label(dialogue.text);
        if ui.button("Continue").clicked() {
            view.add_story_event(StoryEvent::ClickContinueEvent);
        }
    });
}