use std::{collections::HashSet, sync::Arc};

use eframe::{egui::{Context, Pos2, Rect}, glow, Frame};
use events::Event;
use nalgebra_glm::DVec2;
use rendering::Renderers;
use transfer_window_model::{components::ComponentType, storage::entity_allocator::Entity, Model};
use util::{should_render, should_render_at_time};

use crate::{controller_events::ControllerEvent, resources::Resources};

use self::{camera::Camera, debug::DebugWindowTab, frame_history::FrameHistory, overlay::vessel_editor::VesselEditor, selected::Selected};

mod camera;
mod debug;
mod events;
mod expiry;
mod frame_history;
mod misc;
mod overlay;
pub(crate) mod rendering;
mod selected;
mod underlay;
mod util;

pub struct View {
    gl: Arc<glow::Context>,
    model: Model,
    context: Context,
    screen_rect: Rect,
    events: Vec<Event>,
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
    pointer_over_ui: bool
}

impl View {
    pub fn new(gl: Arc<glow::Context>, model: Model, context: Context, resources: Arc<Resources>, focus: Option<Entity>) -> Self {
        let screen_rect = context.screen_rect();
        let events = vec![];
        let mut camera = Camera::new();
        camera.set_focus(focus);
        let renderers = Renderers::new(&resources, &gl, context.screen_rect());
        let selected = Selected::None;
        let right_click_menu = None;
        let vessel_editor = None;
        let frame_history = FrameHistory::default();
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::System;
        let icon_captured_scroll = false;
        let pointer_over_ui = false;
        Self { gl, model, context, screen_rect, events, camera, resources, renderers, selected, right_click_menu, vessel_editor, frame_history, debug_window_open, debug_window_tab, icon_captured_scroll, pointer_over_ui }
    }

    pub fn update(&mut self, context: &Context, frame: &Frame, dt: f64) -> Vec<ControllerEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        self.context = context.clone();
        self.screen_rect = self.context.screen_rect();
        self.frame_history.update(self.context.input(|i| i.time), frame.info().cpu_usage);
        misc::update(self);
        expiry::update(self);
        let is_mouse_over_any_icon = underlay::draw(self);
        overlay::draw(self, is_mouse_over_any_icon);
        debug::draw(self);
        self.pointer_over_ui = context.is_pointer_over_area();
        rendering::update(self);
        events::update(self);
        self.model.update(dt);
        vec![]
    }

    fn is_selected(&self, entity: Entity) -> bool {
        if let Some(selected) = self.selected.entity(&self.model) {
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

    pub fn entities_should_render(&self, with_component_types: Vec<ComponentType>) -> HashSet<Entity> {
        self.model.entities(with_component_types)
            .iter()
            .filter(|entity| should_render(self, **entity))
            .copied()
            .collect()
    }

    pub fn entities_should_render_at_time(&self, with_component_types: Vec<ComponentType>, time: f64) -> HashSet<Entity> {
        self.model.entities(with_component_types)
            .iter()
            .filter(|entity| should_render_at_time(self, **entity, time))
            .copied()
            .collect()
    }

    pub fn window_space_to_world_space(&mut self, window_coords: Pos2) -> DVec2 {
        let offset_x = f64::from(window_coords.x - (self.screen_rect.width() / 2.0)) / self.camera.zoom();
        let offset_y = f64::from((self.screen_rect.height() / 2.0) - window_coords.y) / self.camera.zoom();
        self.camera.translation(&self.model) + DVec2::new(offset_x, offset_y)
    }

    #[allow(unused)]
    pub fn world_space_to_window_space(&mut self, world_coords: DVec2) -> Pos2 {
        let offset = world_coords - self.camera.translation(&self.model);
        let window_coords_x =  (offset.x * self.camera.zoom()) as f32 + 0.5 * self.screen_rect.width();
        let window_coords_y = -(offset.y * self.camera.zoom()) as f32 - 0.5 * self.screen_rect.height();
        Pos2::new(window_coords_x, window_coords_y)
    }
}

impl Drop for View {
    fn drop(&mut self) {
        self.renderers.destroy(&self.gl);
    }
}