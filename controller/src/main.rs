use std::{sync::Arc, time::Instant};

use eframe::{egui::{Context, FontData, FontDefinitions, FontFamily}, glow, run_native, App, CreationContext, Frame, NativeOptions, Renderer};
use log::{debug, error, info};
use transfer_window_model::Model;
use transfer_window_view::{events::Event, menu::Scene, View};

use crate::event_handler::{adjust_burn, adjust_fire_torpedo, cancel_last_event, create_burn, create_fire_torpedo, decrease_time_step_level, destroy, enable_torpedo_guidance, increase_time_step_level, load_game, new_game, quit, save_game, set_slot, set_target, start_warp, toggle_paused};

mod event_handler;

struct Controller {
    gl: Arc<glow::Context>,
    model: Option<Model>,
    view: View,
    last_frame: Instant,
}

impl Controller {
    pub fn init(creation_context: &CreationContext) -> Box<dyn eframe::App> {
        info!("Initialising controller");

        // https://github.com/amPerl/egui-phosphor/blob/main/src/lib.rs
        let mut font_definition = FontDefinitions::default();
        font_definition.font_data.insert("mdi".into(), FontData::from_static(include_bytes!("../resources/fonts/MaterialIcons-Regular.ttf")));
        if let Some(font_keys) = font_definition.families.get_mut(&FontFamily::Proportional) {
            debug!("Loading proportional MDI font");
            font_keys.push("mdi".into());
        }
        if let Some(font_keys) = font_definition.families.get_mut(&FontFamily::Monospace) {
            debug!("Loading monospace MDI font");
            font_keys.push("mdi".into());
        }
        creation_context.egui_ctx.set_fonts(font_definition);

        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let gl = creation_context.gl.as_ref().unwrap().clone();
        let model = None;
        let view = View::MenuScene(Scene::default());
        let last_frame = Instant::now();
        Box::new(Self { gl, model, view, last_frame })
    }

    pub fn model(&self) -> &Model {
        if let Some(model) = self.model.as_ref() {
            return model;
        }
        error!("No model is loaded");
        panic!("Unrecoverable error");
    }

    fn model_mut(&mut self) -> &mut Model {
        if let Some(model) = self.model.as_mut() {
            return model;
        }
        error!("No model is loaded");
        panic!("Unrecoverable error");
    }

    fn handle_events(&mut self, mut events: Vec<Event>, frame: &mut Frame) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Event handling");

        while let Some(event) = events.pop() {
            debug!("Handling event {:?}", event);
            match event {
                Event::Quit => quit(frame),
                Event::NewGame => new_game(self),
                Event::SaveGame { name } => save_game(self, name.as_str()),
                Event::LoadGame { name } => load_game(self, name.as_str()),
                Event::TogglePaused => toggle_paused(self),
                Event::IncreaseTimeStepLevel => increase_time_step_level(self),
                Event::DecreaseTimeStepLevel => decrease_time_step_level(self),
                Event::StartWarp { end_time } => start_warp(self, end_time),
                Event::CreateBurn { entity, time } => create_burn(self, entity, time),
                Event::AdjustBurn { entity, time, amount } => adjust_burn(self, entity, time, amount),
                Event::Destroy { entity } => destroy(self, entity),
                Event::SetTarget { entity, target } => set_target(self, entity, target),
                Event::SetSlot { entity, slot_location, slot } => set_slot(self, entity, slot_location, slot),
                Event::CreateFireTorpedo { entity, slot_location, time } => create_fire_torpedo(self, entity, slot_location, time),
                Event::AdjustFireTorpedo { entity, time, amount } => adjust_fire_torpedo(self, entity, time, amount),
                Event::CancelLastTimelineEvent { entity } => cancel_last_event(self, entity),
                Event::EnableTorpedoGuidance { entity, time } => enable_torpedo_guidance(self, entity, time),
            }
        }
    }
}

impl App for Controller {
    fn update(&mut self, context: &Context, frame: &mut Frame) {
        #[cfg(feature = "profiling")]
        tracy_client::frame_mark();
        
        let dt = self.last_frame.elapsed().as_secs_f64();
        self.last_frame = Instant::now();

        let events = match &mut self.view {
            View::GameScene(scene) => scene.update(self.model.as_ref().unwrap(), context, frame),
            View::MenuScene(scene) => scene.update(context),
        };

        self.handle_events(events, frame);

        if let Some(model) = &mut self.model {
            model.update(dt);
        }

        context.request_repaint(); // Without this, the context will only update when some input changes
    }
}

fn main() {
    env_logger::init();

    let options = NativeOptions {
        renderer: Renderer::Glow,
        multisampling: 8,
        ..Default::default()
    };

    let _ = run_native("Transfer Window", options, Box::new(Controller::init));
}
