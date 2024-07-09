use eframe::egui::{Color32, CursorIcon, Pos2, Response, RichText, Ui, Widget, Window};

use crate::{game::{storyteller::story::story_event::StoryEvent, View}, styles};

use super::widgets::custom_image::CustomImage;

#[derive(Debug, Clone)]
enum DialogueComponent {
    Normal(&'static str),
    Bold(&'static str),
}

#[derive(Debug, Clone)]
pub struct Dialogue {
    character: &'static str,
    components: Vec<DialogueComponent>,
}

impl Dialogue {
    pub fn new(character: &'static str) -> Self {
        let components = vec![];
        Self { character, components }
    }

    pub fn normal(mut self, text: &'static str ) -> Self {
        self.components.push(DialogueComponent::Normal(text));
        self
    }
    pub fn bold(mut self, text: &'static str ) -> Self {
        self.components.push(DialogueComponent::Bold(text));
        self
    }
}

impl Widget for Dialogue {
    fn ui(self, ui: &mut Ui) -> Response {
        // https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/code_editor.rs
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for component in &self.components {
                match component {
                    DialogueComponent::Normal(text) => ui.label(RichText::new(*text).monospace().color(Color32::WHITE)),
                    DialogueComponent::Bold(text) => ui.label(RichText::new(*text).monospace().color(Color32::GOLD)),
                };
            }
        }).response
    }
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update dialogue");

    let Some(dialogue) = view.dialogue.clone() else {
        return;
    };

    Window::new("Dialogue")
            .resizable(false)
            .collapsible(false)
            .movable(true)
            .title_bar(false)
            .default_pos(Pos2::new(view.context.screen_rect().width() / 2.0 - 500.0 / 2.0, 80.0))
            .show(&view.context.clone(), |ui| {
        ui.vertical(|ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add(CustomImage::new(view, &(dialogue.character.to_string() + ".character"), 100.0));
                ui.vertical(|ui| {
                    ui.set_width(350.0);
                    ui.add(dialogue);
                });
                ui.add_space(7.0);
            });
    
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                styles::DialogueContinueButton::apply(ui);
                let response = ui.button("Continue");
                if response.hovered() {
                    view.context.set_cursor_icon(CursorIcon::PointingHand);
                }
                if response.clicked() {
                    view.add_story_event(StoryEvent::ClickContinueEvent);
                }
            });
        });
    });
}