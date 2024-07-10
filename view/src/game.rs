use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use eframe::egui::{Context, Pos2, Rect};
use eframe::{glow, Frame};
use events::{ModelEvent, ViewEvent};
use nalgebra_glm::DVec2;
use overlay::dialogue::Dialogue;
use overlay::objectives::Objective;
use rendering::Renderers;
use storyteller::story::Story;
use transfer_window_model::components::ComponentType;
use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::story_event::StoryEvent;
use transfer_window_model::Model;
use util::{should_render, should_render_at_time};

use self::camera::Camera;
use self::debug::DebugWindowTab;
use self::frame_history::FrameHistory;
use self::overlay::vessel_editor::VesselEditor;
use self::selected::Selected;
use crate::controller_events::ControllerEvent;
use crate::resources::Resources;

mod animation;
pub(crate) mod camera;
mod debug;
mod events;
mod expiry;
mod frame_history;
mod misc;
pub(crate) mod overlay;
pub(crate) mod rendering;
mod selected;
pub mod storyteller;
mod underlay;
mod util;

pub struct ViewConfig {
    apsis_icons: bool,
    selected: bool,
    explorer: bool,
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self {
            apsis_icons: true,
            selected: true,
            explorer: true,
        }
    }
}

pub struct View {
    gl: Arc<glow::Context>,
    model: Model,
    story: Story,
    config: ViewConfig,
    context: Context,
    previous_screen_rect: Rect,
    screen_rect: Rect,
    controller_events: Arc<Mutex<Vec<ControllerEvent>>>,
    model_events: Arc<Mutex<Vec<ModelEvent>>>,
    view_events: Arc<Mutex<Vec<ViewEvent>>>,
    story_events: Arc<Mutex<Vec<StoryEvent>>>,
    camera: Camera,
    resources: Arc<Resources>,
    renderers: Renderers,
    selected: Selected,
    right_click_menu: Option<Entity>,
    vessel_editor: Option<VesselEditor>,
    dialogue: Option<Dialogue>,
    frame_history: FrameHistory,
    debug_window_open: bool,
    debug_window_tab: DebugWindowTab,
    pointer_over_ui: bool,
    pointer_over_icon: bool,
    objectives: Vec<Objective>,
}

impl View {
    pub fn new(
        gl: Arc<glow::Context>,
        model: Model,
        story: Story,
        context: Context,
        resources: Arc<Resources>,
        config: ViewConfig,
        focus: Option<Entity>,
    ) -> Self {
        let previous_screen_rect = context.screen_rect();
        let screen_rect = context.screen_rect();
        let controller_events = Arc::new(Mutex::new(vec![]));
        let model_events = Arc::new(Mutex::new(vec![]));
        let view_events = Arc::new(Mutex::new(vec![]));
        let story_events = Arc::new(Mutex::new(vec![]));
        let mut camera = Camera::new();
        if let Some(focus) = focus {
            camera.set_focus(focus, model.absolute_position(focus));
        }
        let renderers = Renderers::new(&resources, &gl, context.screen_rect());
        let selected = Selected::None;
        let right_click_menu = None;
        let vessel_editor = None;
        let dialogue = None;
        let frame_history = FrameHistory::default();
        let debug_window_open = false;
        let debug_window_tab = DebugWindowTab::Model;
        let pointer_over_ui = false;
        let pointer_over_icon = false;
        let objectives = vec![];
        Self {
            gl,
            model,
            story,
            config,
            context,
            previous_screen_rect,
            screen_rect,
            controller_events,
            model_events,
            view_events,
            story_events,
            camera,
            resources,
            renderers,
            selected,
            right_click_menu,
            vessel_editor,
            dialogue,
            frame_history,
            debug_window_open,
            debug_window_tab,
            pointer_over_ui,
            pointer_over_icon,
            objectives,
        }
    }

    fn update_camera_focus_position(&mut self) {
        if let Some(focus) = self.camera.focus() {
            self.set_camera_focus(focus);
        }
    }

    fn draw_ui(&self) {
        misc::update(self);
        underlay::draw(self);
        overlay::draw(self);
        debug::draw(self);
    }

    fn post_draw_ui(&mut self) {
        self.pointer_over_ui = self.context.is_pointer_over_area();
        self.pointer_over_icon = false;
    }

    fn draw_underlay(&self) {
        rendering::update(self);
    }

    pub fn update(&mut self, context: &Context, frame: &Frame, dt: f64) -> Vec<ControllerEvent> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("View update");
        self.controller_events.lock().unwrap().clear();
        self.context = context.clone();
        self.previous_screen_rect = self.screen_rect;
        self.screen_rect = self.context.screen_rect();
        self.frame_history.update(self.context.input(|i| i.time), frame.info().cpu_usage);
        self.update_animation(dt);
        self.update_camera_focus_position();
        self.draw_ui();
        self.post_draw_ui();
        self.draw_underlay();
        self.handle_events();
        self.story_events.lock().unwrap().extend(self.model.update(dt));
        expiry::update(self);
        self.controller_events.lock().unwrap().clone()
    }

    pub(crate) fn add_controller_event(&self, event: ControllerEvent) {
        self.controller_events.lock().unwrap().push(event);
    }

    pub(crate) fn add_model_event(&self, event: ModelEvent) {
        self.model_events.lock().unwrap().push(event);
    }

    pub(crate) fn add_view_event(&self, event: ViewEvent) {
        self.view_events.lock().unwrap().push(event);
    }

    pub(crate) fn add_story_event(&self, event: StoryEvent) {
        self.story_events.lock().unwrap().push(event);
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

    pub(crate) fn entities_should_render(
        &self,
        with_component_types: Vec<ComponentType>,
    ) -> HashSet<Entity> {
        self.model
            .entities(with_component_types)
            .iter()
            .filter(|entity| should_render(self, **entity))
            .copied()
            .collect()
    }

    #[allow(unused)]
    pub(crate) fn entities_should_render_at_time(
        &self,
        with_component_types: Vec<ComponentType>,
        time: f64,
    ) -> HashSet<Entity> {
        self.model
            .entities(with_component_types)
            .iter()
            .filter(|entity| should_render_at_time(self, **entity, time))
            .copied()
            .collect()
    }

    pub(crate) fn set_camera_focus(&mut self, focus: Entity) {
        let focus_position = self.model.absolute_position(focus);
        self.camera.set_focus(focus, focus_position);
    }

    pub(crate) fn window_space_to_world_space(&self, window_coords: Pos2) -> DVec2 {
        let offset_x =
            f64::from(window_coords.x - (self.screen_rect.width() / 2.0)) / self.camera.zoom();
        let offset_y =
            f64::from((self.screen_rect.height() / 2.0) - window_coords.y) / self.camera.zoom();
        self.camera.translation() + DVec2::new(offset_x, offset_y)
    }

    pub(crate) fn world_space_to_window_space(&self, world_coords: DVec2) -> Pos2 {
        let offset = world_coords - self.camera.translation();
        let window_coords_x =
            (offset.x * self.camera.zoom()) as f32 + 0.5 * self.screen_rect.width();
        let window_coords_y =
            -(offset.y * self.camera.zoom()) as f32 - 0.5 * self.screen_rect.height();
        Pos2::new(window_coords_x, window_coords_y)
    }
}

impl Drop for View {
    fn drop(&mut self) {
        self.renderers.destroy(&self.gl);
    }
}
