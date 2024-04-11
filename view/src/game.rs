use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, Context}, glow};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::events::Event;

use self::{camera::Camera, debug::DebugWindowTab, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer}, resources::Resources, underlay::selected::Selected};

mod camera;
mod debug;
mod input;
mod overlay;
mod rendering;
mod resources;

mod underlay;
mod util;

pub struct Scene {
    camera: Camera,
    object_renderer: Arc<Mutex<GeometryRenderer>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
    selected: Selected,
    debug_window_open: bool,
    debug_window_tab: DebugWindowTab,
}

impl Scene {
    pub fn new(gl: &Arc<glow::Context>, focus: Option<Entity>) -> Self {
        let mut resources = Resources::new();
        let mut camera = Camera::new();
        camera.set_focus(focus);
        let object_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let texture_renderers = resources.build_renderers(gl);
        let selected_point = Selected::None;
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::Overview;
        Self { camera, object_renderer, segment_renderer, texture_renderers, selected: selected_point, debug_window_open, debug_window_tab }
    }

    pub fn update(&mut self, model: &Model, context: &Context) -> Vec<Event> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        let mut events = vec![];
        
        CentralPanel::default().show(context, |ui| {
            input::update(self, context, &mut events);
            underlay::draw(self, model, context);
            overlay::draw(self, model, context, &mut events);
            debug::draw(self, model, context);
            rendering::update(self, model, context);
            
            if ui.button("Save").clicked() {
                events.push(Event::SaveGame { name: "test".to_owned() });
            }
        });

        events
    }
}