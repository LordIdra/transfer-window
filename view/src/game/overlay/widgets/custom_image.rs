use std::sync::{Arc, Mutex};

use eframe::{egui::{PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget}, egui_glow::CallbackFn, emath::RectTransform, glow};

use crate::game::{camera::Camera, rendering::screen_texture_renderer::ScreenTextureRenderer, View};

pub struct CustomImage {
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    texture: glow::Texture,
    screen_rect: Rect,
    width: i32,
    height: i32,
    sense: Sense,
    alpha: f32,
}

impl CustomImage {
    pub fn new(view: &View, texture_name: &str, size: i32) -> Self {
        let renderer = view.renderers.screen_texture_renderer();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let width = size;
        let height = size;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let alpha = 1.0;
        Self { renderer, texture, screen_rect, width, height, sense, alpha }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_from_parts(renderer: Arc<Mutex<ScreenTextureRenderer>>, texture: glow::Texture, screen_rect: Rect, width: i32, height: i32, sense: Sense, alpha: f32) -> Self {
        Self { renderer, texture, screen_rect, width, height, sense, alpha }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }
}

impl Widget for CustomImage {
    fn ui(self, ui: &mut Ui) -> Response {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw custom image");

        let (response, painter) = ui.allocate_painter(Vec2::new(self.width as f32, self.height as f32), self.sense);
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let corner = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(0.0, 0.0)));
        let renderer = self.renderer.clone();

        let callback = Arc::new(CallbackFn::new(move |_info, painter| {
            renderer.lock().unwrap().render(painter.gl(), self.texture, self.screen_rect, corner, self.width, self.height, self.alpha);
        }));

        painter.add(PaintCallback { rect: self.screen_rect, callback});
        
        response
    }
}
