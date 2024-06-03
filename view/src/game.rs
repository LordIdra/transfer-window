use std::{collections::HashSet, sync::Arc};

use eframe::{egui::Context, glow, Frame};
use renderers::Renderers;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};
use util::{should_render, should_render_at_time};

use crate::{events::Event, resources::Resources};

use self::{camera::Camera, debug::DebugWindowTab, frame_history::FrameHistory, overlay::vessel_editor::VesselEditor, selected::Selected};

pub mod camera;
mod debug;
mod expiry;
mod frame_history;
mod misc;
mod overlay;
mod renderers;
mod selected;
mod underlay;
mod util;

pub struct Scene {
    camera: Camera,
    resources: Arc<Resources>,
    renderers: Renderers,
    selected: Selected,
    right_click_menu: Option<Entity>,
    vessel_editor: Option<VesselEditor>,
    frame_history: FrameHistory,
    debug_window_open: bool,
    debug_window_tab: DebugWindowTab,
    icon_captured_scroll: bool,
    pointer_over_ui_last_frame: bool
}

impl Scene {
    pub fn new(gl: Arc<glow::Context>, context: &Context, resources: Arc<Resources>, focus: Option<Entity>) -> Self {
        let mut camera = Camera::new();
        camera.set_focus(focus);
        let renderers = Renderers::new(&resources, gl, context.screen_rect());
        let selected = Selected::None;
        let right_click_menu = None;
        let vessel_editor = None;
        let frame_history = FrameHistory::default();
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::Overview;
        let icon_captured_scroll = false;
        let pointer_over_ui_last_frame = false;
        Self { camera, resources, renderers, selected, right_click_menu, vessel_editor, frame_history, debug_window_open, debug_window_tab, icon_captured_scroll, pointer_over_ui_last_frame }
    }

    pub fn update(&mut self, model: &Model, context: &Context, frame: &Frame) -> Vec<Event> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        let mut events = vec![];
        
        self.frame_history.update(context.input(|i| i.time), frame.info().cpu_usage);
        misc::update(self, model, context, &mut events);
        expiry::update(self, model);
        let is_mouse_over_any_icon = underlay::draw(self, model, context, &mut events);
        overlay::draw(self, model, context, is_mouse_over_any_icon, &mut events);
        debug::draw(self, model, context);
        self.pointer_over_ui_last_frame = context.is_pointer_over_area();
        renderers::update(self, model, context);

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

    pub fn entities_should_render(&self, model: &Model, with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        model.entities(with_component_types)
            .iter()
            .filter(|entity| should_render(self, model, **entity))
            .copied()
            .collect()
    }

    pub fn entities_should_render_at_time(&self, model: &Model, with_component_types: Vec<ComponentType>, time: f64) -> HashSet<Entity> {
        model.entities(with_component_types)
            .iter()
            .filter(|entity| should_render_at_time(self, model, **entity, time))
            .copied()
            .collect()
    }
}