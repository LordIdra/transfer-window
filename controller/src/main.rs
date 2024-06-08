use std::{sync::Arc, time::Instant};

use eframe::{egui::Context, glow, run_native, App, CreationContext, Frame, NativeOptions, Renderer};
use event_handler::{load_game, new_game, quit};
use log::{debug, info};
use transfer_window_view::{controller_events::ControllerEvent, menu, resources::Resources, Scene};

mod event_handler;

struct Controller {
    gl: Arc<glow::Context>,
    resources: Arc<Resources>,
    scene: Scene,
    last_frame: Instant,
}

impl Controller {
    pub fn init(creation_context: &CreationContext) -> Box<dyn eframe::App> {
        info!("Initialising controller");

        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let gl = creation_context.gl.as_ref().unwrap().clone();
        let resources = Arc::new(Resources::new(&creation_context.egui_ctx, creation_context.gl.as_ref().unwrap()));
        let view = Scene::Menu(menu::View::default());
        let last_frame = Instant::now();
        Box::new(Self { gl, resources, scene: view, last_frame })
    }

    fn handle_events(&mut self, mut events: Vec<ControllerEvent>, context: &Context) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Event handling");

        while let Some(event) = events.pop() {
            debug!("Handling event {:?}", event);
            match event {
                ControllerEvent::Quit => quit(context),
                ControllerEvent::NewGame => new_game(self, context),
                ControllerEvent::LoadGame { name } => load_game(self, context, name.as_str()),
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

        let events = match &mut self.scene {
            Scene::Game(view) => view.update(context, frame, dt),
            Scene::Menu(view) => view.update(context),
        };

        self.handle_events(events, context);

        context.request_repaint(); // Without this, the context will only update when some input changes
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.resources.destroy(&self.gl);
    }
}

fn main() {
    env_logger::init();

    let options = NativeOptions {
        renderer: Renderer::Glow,
        ..Default::default()
    };

    let _ = run_native("Transfer Window", options, Box::new(Controller::init));
}
