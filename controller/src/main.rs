use eframe::{egui::Context, run_native, App, CreationContext, Frame, NativeOptions, Renderer};
use transfer_window_model::Model;
use transfer_window_view::View;

struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new(creation_context: &CreationContext) -> Box<dyn eframe::App> {
        let model = Model::new();
        let view = View::new(creation_context.gl.as_ref().unwrap().clone());
        Box::new(Self { model, view })
    }
}

impl App for Controller {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        self.view.update(context, &self.model);
        context.request_repaint(); // Without this, the context will only update when some input changes
    }
}

fn main() {
    let options = NativeOptions {
        renderer: Renderer::Glow,
        multisampling: 16,
        ..Default::default()
    };

    let _ = run_native("Transfer Window", options, Box::new(Controller::new));
}
