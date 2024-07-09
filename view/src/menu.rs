use std::{collections::HashSet, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, Context, CursorIcon, Key, Rect, Sense, Ui, Vec2, Window}, glow};
use log::trace;

use crate::{controller_events::ControllerEvent, game::{overlay::widgets::custom_image::CustomImage, rendering::screen_texture_renderer::ScreenTextureRenderer, storyteller::stories::{story_01_welcome::Story01Welcome, StoryBuilder}}, resources::Resources};

impl CustomImage {
    pub fn new_menu(view: &View, texture_name: &str, width: f32, height: f32) -> Self {
        let renderer = view.screen_texture_renderer.clone();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let padding = 0.0;
        let alpha = 1.0;
        Self::new_from_parts(renderer, texture, screen_rect, width, height, sense, padding, alpha)
    }
}

pub struct View {
    gl: Arc<glow::Context>,
    previous_screen_rect: Rect,
    screen_rect: Rect,
    resources: Arc<Resources>,
    screen_texture_renderer: Arc<Mutex<ScreenTextureRenderer>>,
    debug_window_open: bool,
}

impl View {
    pub fn new(resources: Arc<Resources>, context: &Context, gl: Arc<glow::Context>) -> Self {
        let previous_screen_rect = context.screen_rect();
        let screen_rect = context.screen_rect();
        let screen_texture_renderer = Arc::new(Mutex::new(ScreenTextureRenderer::new(&gl, screen_rect)));
        let debug_window_open = false;
        Self { gl, previous_screen_rect, screen_rect, resources, screen_texture_renderer, debug_window_open }
    }

    fn draw_level(&self, context: &Context, ui: &mut Ui, events: &mut Vec<ControllerEvent>, completed_levels: &HashSet<String>, level: &str, story_builder: Box<dyn StoryBuilder>) {
        let mut level = level.to_string();
        let prerequisite_met = story_builder.prerequisite().map_or_else(|| true, |prerequisite| completed_levels.contains(&prerequisite));
        let (rect, _) = ui.allocate_exact_size(Vec2::new(300.0, 150.0), Sense::click());
        let hovered = ui.rect_contains_pointer(rect);
        let clicked = hovered && ui.input(|input| input.pointer.primary_clicked());
        if prerequisite_met && clicked {
            events.push(ControllerEvent::NewGame { story_builder });
        }
        if prerequisite_met && hovered {
            context.set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.allocate_ui_at_rect(rect, |ui| {
            if completed_levels.contains(&level) {
                level += "-complete"
            }
            let mut image = CustomImage::new_menu(self, &level, 300.0, 150.0);
            if !hovered {
                image = image.with_alpha(0.7);
            }
            if !prerequisite_met {
                image = image.with_alpha(0.4);
            }
            ui.add(image);
        });

        ui.add_space(10.0);
    }

    pub fn update(&mut self, context: &Context, completed_levels: HashSet<String>) -> Vec<ControllerEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");

        self.previous_screen_rect = self.screen_rect;
        self.screen_rect = context.screen_rect();

        if context.input(|input| input.key_pressed(Key::F12)) {
            self.debug_window_open = !self.debug_window_open;
            trace!("Menu debug window = {}", self.debug_window_open);
        }

        // Make sure to regenerate framebuffer with new size if window resized
        if context.screen_rect() != self.previous_screen_rect {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Resize buffers");
            self.screen_texture_renderer.lock().unwrap().resize(&self.gl, context.screen_rect());
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
            ui.vertical_centered(|ui| {
                ui.add(CustomImage::new_menu(self, "title", 840.0, 160.0));
            });
            ui.horizontal(|ui| {
                ui.add_space(100.0);
                ui.vertical(|ui| {
                    ui.add(CustomImage::new_menu(self, "title-1", 215.0, 70.0));
                    ui.horizontal(|ui| {
                        self.draw_level(context, ui, &mut events, &completed_levels, "1-01", Box::new(Story01Welcome));
                        self.draw_level(context, ui, &mut events, &completed_levels, "1-02", Box::new(Story01Welcome));
                    });
                    // ui.add_space(15.0);
                    // ui.horizontal(|ui| {

                    // });
                })
            })
        });
        events
    }
}

impl Drop for View {
    fn drop(&mut self) {
        self.screen_texture_renderer.lock().unwrap().destroy(&self.gl);
    }
}