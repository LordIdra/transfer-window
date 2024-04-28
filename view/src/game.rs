use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::Context, glow, Frame};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::events::Event;

use self::{camera::Camera, debug::DebugWindowTab, frame_history::FrameHistory, overlay::vessel::VesselEditor, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer}, resources::Resources, underlay::selected::Selected};

mod camera;
mod debug;
mod expiry;
mod frame_history;
mod input;
mod overlay;
mod rendering;
mod resources;
mod underlay;
mod util;

pub struct Scene {
    camera: Camera,
    resources: Resources,
    object_renderer: Arc<Mutex<GeometryRenderer>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
    selected: Selected,
    right_click_menu: Option<Entity>,
    vessel_editor: Option<VesselEditor>,
    frame_history: FrameHistory,
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
        let selected = Selected::None;
        let right_click_menu = None;
        let vessel_editor = None;
        let frame_history = FrameHistory::default();
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::Overview;
        Self { camera, resources, object_renderer, segment_renderer, texture_renderers, selected, right_click_menu, vessel_editor, frame_history, debug_window_open, debug_window_tab }
    }

    pub fn update(&mut self, model: &Model, context: &Context, frame: &Frame) -> Vec<Event> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        let mut events = vec![];
        
        self.frame_history.update(context.input(|i| i.time), frame.info().cpu_usage);
        expiry::update(self, model);
        input::update(self, context, &mut events);
        underlay::draw(self, model, context, &mut events);
        overlay::draw(self, model, context, &mut events);
        debug::draw(self, model, context);
        rendering::update(self, model, context);

        events
    }
}