use eframe::egui::{Color32, CursorIcon, Pos2, Response, RichText, Ui, Window};
use transfer_window_model::story_event::StoryEvent;

use crate::{game::View, styles};

use super::widgets::custom_image::CustomImage;

#[derive(Debug, Clone)]
enum DialogueComponent {
    Normal(&'static str),
    Bold(&'static str),
    Image(&'static str),
}

#[derive(Debug, Clone)]
pub struct Dialogue {
    character: &'static str,
    components: Vec<DialogueComponent>,
    has_continue: bool,
}

impl Dialogue {
    pub fn new(character: &'static str) -> Self {
        let components = vec![];
        let has_continue = false;
        Self { character, components, has_continue }
    }

    pub fn normal(mut self, text: &'static str ) -> Self {
        self.components.push(DialogueComponent::Normal(text));
        self
    }

    pub fn bold(mut self, text: &'static str ) -> Self {
        self.components.push(DialogueComponent::Bold(text));
        self
    }

    pub fn image(mut self, text: &'static str ) -> Self {
        self.components.push(DialogueComponent::Image(text));
        self
    }

    pub fn with_continue(mut self) -> Self {
        self.has_continue = true;
        self
    }

    pub fn draw(self, view: &View, ui: &mut Ui) -> Response {
        // https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/code_editor.rs
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for component in &self.components {
                match component {
                    DialogueComponent::Normal(text) => ui.label(RichText::new(*text).size(14.0).color(Color32::WHITE)),
                    DialogueComponent::Bold(text) => ui.label(RichText::new(*text).size(14.0).color(Color32::GOLD)),
                    DialogueComponent::Image(texture) => ui.add(CustomImage::new(view, texture, 14.0)),
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

    let has_continue = dialogue.has_continue;

    Window::new("Dialogue")
            .resizable(false)
            .collapsible(false)
            .movable(true)
            .title_bar(false)
            .default_pos(Pos2::new(view.context.screen_rect().width() / 2.0 - 500.0 / 2.0, 80.0))
            .show(&view.context.clone(), |ui| {
        ui.vertical(|ui| {
            ui.set_min_height(160.0);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add(CustomImage::new(view, &(dialogue.character.to_string() + ".character"), 100.0));
                ui.vertical(|ui| {
                    ui.set_width(350.0);
                    dialogue.draw(view, ui);
                });
                ui.add_space(7.0);
            });
    
            if has_continue {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    styles::DialogueContinueButton::apply(ui);
                    let response = ui.button(RichText::new("Continue").strong().monospace().size(12.0));
                    if response.hovered() {
                        view.context.set_cursor_icon(CursorIcon::PointingHand);
                    }
                    if response.clicked() {
                        view.add_story_event(StoryEvent::ClickContinue);
                    }
                    ui.add_space(5.0);
                });
            }
        });
    });
}