use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::Context, glow, Frame};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::events::Event;

use self::{camera::Camera, debug::DebugWindowTab, frame_history::FrameHistory, overlay::vessel_editor::VesselEditor, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer, object_renderer::ObjectRenderer}, resources::Resources, underlay::selected::Selected};

mod camera;
mod debug;
mod expiry;
mod frame_history;
mod misc;
mod overlay;
mod rendering;
mod resources;
mod underlay;
mod util;

pub struct Scene {
    camera: Camera,
    resources: Resources,
    object_renderers: HashMap<String, Arc<Mutex<ObjectRenderer>>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
    selected: Selected,
    right_click_menu: Option<Entity>,
    vessel_editor: Option<VesselEditor>,
    frame_history: FrameHistory,
    debug_window_open: bool,
    debug_window_tab: DebugWindowTab,
    icon_captured_scroll: bool,
}

impl Scene {
    pub fn new(gl: &Arc<glow::Context>, focus: Option<Entity>) -> Self {
        let mut resources = Resources::new();
        let mut camera = Camera::new();
        camera.set_focus(focus);
        let object_renderers = resources.build_object_renderers(gl);
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let texture_renderers = resources.build_texture_renderers(gl);
        let selected = Selected::None;
        let right_click_menu = None;
        let vessel_editor = None;
        let frame_history = FrameHistory::default();
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::Overview;
        let icon_captured_scroll = false;
        Self { camera, resources, object_renderers, segment_renderer, texture_renderers, selected, right_click_menu, vessel_editor, frame_history, debug_window_open, debug_window_tab, icon_captured_scroll }
    }

    pub fn update(&mut self, model: &Model, context: &Context, frame: &Frame) -> Vec<Event> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        let mut events = vec![];
        
        self.frame_history.update(context.input(|i| i.time), frame.info().cpu_usage);
        misc::update(self, model, context, &mut events);
        expiry::update(self, model);
        underlay::draw(self, model, context, &mut events);
        overlay::draw(self, model, context, &mut events);
        debug::draw(self, model, context);
        rendering::update(self, model, context);

        events
    }

    fn is_selected(&self, model: &Model, entity: Entity) -> bool {
        if let Some(selected) = self.selected.entity(model) {
            selected == entity
        } else {
            false
        }
    }

    fn toggle_right_click_menu(&mut self, right_clicked: Entity) {
        if let Some(entity) = self.right_click_menu {
            if entity == right_clicked {
                self.right_click_menu = None;
                return;
            }
        }
        self.right_click_menu = Some(right_clicked);
    }
}