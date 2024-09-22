use std::{error::Error, fs::{create_dir_all, File}, sync::Arc, time::Instant};

use completed_levels::CompletedLevels;
use eframe::{egui::{Context, Key, ViewportBuilder, ViewportCommand}, glow::{self, HasContext, RENDERER, SHADING_LANGUAGE_VERSION, VERSION}, run_native, App, CreationContext, Frame, NativeOptions};
use event_handler::{exit_level, finish_level, load_game, new_game, quit};
use log::{debug, info};
use sysinfo::System;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use transfer_window_view::{controller_events::ControllerEvent, menu, resources::Resources, Scene};

mod event_handler;
mod completed_levels;

type DynError = Box<dyn Error + Send + Sync>; // why didn't egui just make this public?

struct Controller {
    gl: Arc<glow::Context>,
    resources: Arc<Resources>,
    completed_levels: CompletedLevels,
    scene: Scene,
    last_frame: Instant,
    load_menu: bool, // menu loading must occur RIGHT AT THE BEGINNING of a frame before any updates
}

fn log_gl_info(gl: &Arc<glow::Context>) {
    info!("OpenGL version: {}", unsafe { gl.get_parameter_string(VERSION) });
    info!("OpenGL renderer: {}", unsafe { gl.get_parameter_string(RENDERER) });
    info!("GLSL version: {}", unsafe { gl.get_parameter_string(SHADING_LANGUAGE_VERSION) });
}

impl Controller {
    #[allow(clippy::unnecessary_wraps)]
    pub fn init(creation_context: &CreationContext) -> Result<Box<dyn App>, DynError> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Controller initialisation");

        info!("Initialising controller");

        log_gl_info(creation_context.gl.as_ref().unwrap());

        creation_context.egui_ctx.send_viewport_cmd(ViewportCommand::Maximized(true));

        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let gl = creation_context.gl.as_ref().unwrap().clone();
        let resources = Arc::new(Resources::new(&creation_context.egui_ctx, &gl));
        let view = Scene::Menu(menu::View::new(resources.clone(), &creation_context.egui_ctx, gl.clone()));
        let last_frame = Instant::now();
        let load_menu = false;
        let completed_levels = CompletedLevels::load();
        Ok(Box::new(Self { gl, resources, scene: view, last_frame, load_menu, completed_levels }))
    }

    fn handle_events(&mut self, mut events: Vec<ControllerEvent>, context: &Context) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Event handling");

        while let Some(event) = events.pop() {
            debug!("Handling controller event {:?}", event);
            match event {
                ControllerEvent::Quit => quit(context),
                ControllerEvent::NewGame { story_builder } => new_game(self, context, &*story_builder),
                ControllerEvent::LoadGame { name } => load_game(self, context, name.as_str()),
                ControllerEvent::FinishLevel { level } => finish_level(self, level),
                ControllerEvent::ExitLevel => exit_level(self),
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

        if self.load_menu {
            self.scene = Scene::Menu(menu::View::new(self.resources.clone(), context, self.gl.clone()));
            self.load_menu = false;
        }

        let events = match &mut self.scene {
            Scene::Game(view) => view.update(context, frame, dt),
            Scene::Menu(view) => view.update(context, self.completed_levels.get()),
        };

        self.handle_events(events, context);

        // toggle fullscreen on f11
        let fullscreen = context.input(|input| input.viewport().fullscreen.is_some_and(|fullscreen| fullscreen));
        if context.input(|input| input.key_pressed(Key::F11)) {
            context.send_viewport_cmd(ViewportCommand::Fullscreen(!fullscreen));
        }

        context.request_repaint(); // Without this, the context will only update when some input changes
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.resources.destroy(&self.gl);
    }
}

fn setup_logging() {
    // A layer that logs events to a file.
    create_dir_all("log").expect("Failed to create log directory");
    let file = File::create("data/log/latest.log").expect("Failed to create file");
    let layer = Layer::new().compact()
        .with_ansi(false)
        .with_writer(Arc::new(file));
    let filter = EnvFilter::builder()
        .parse("debug,transfer_window_controller=trace,transfer_window_view=trace,transfer_window_model=trace")
        .expect("Failed to parse env filter");
    tracing_subscriber::registry()
        .with(layer)
        .with(filter)
        .init();

    info!("Starting application");

    let sys = System::new_all();
    info!("Memory: {} B", sys.total_memory());
    info!("Swap: {} B", sys.total_swap());
    info!("OS: {:?}", System::long_os_version());
    info!("Kernel version: {:?}", System::kernel_version());
    info!("CPU architecture: {:?}", System::cpu_arch());
    for (i, cpu) in sys.cpus().iter().enumerate() {
        info!("CPU #{}: {} @ {} MHz", i, cpu.brand(), cpu.frequency());
    }
}

fn main() {
    setup_logging();

    let options = NativeOptions {
        viewport: ViewportBuilder::default(),
        ..Default::default()
    };

    let _ = run_native("Transfer Window", options, Box::new(Controller::init));
}
