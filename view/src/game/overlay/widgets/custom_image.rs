use std::sync::{Arc, Mutex};

use eframe::{egui::{PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget}, egui_glow::CallbackFn, emath::RectTransform};

use crate::{game::camera::Camera, rendering::screen_texture_renderer::ScreenTextureRenderer};

pub struct CustomImage {
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    screen_rect: Rect,
    size: f32,
    sense: Sense,
    padding: f32,
}

impl CustomImage {
    pub fn new(renderer: Arc<Mutex<ScreenTextureRenderer>>, screen_rect: Rect, size: f32) -> Self {
        let sense = Sense::union(Sense::click(), Sense::hover());
        let padding = 0.0;
        Self { renderer, screen_rect, size, sense, padding }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl Widget for CustomImage {
    fn ui(self, ui: &mut Ui) -> Response {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw custom image");

        let (response, painter) = ui.allocate_painter(Vec2::splat(self.size), self.sense);
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let from = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(self.padding, self.padding)));
        let to = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(self.size - self.padding, self.size - self.padding)));
        let renderer = self.renderer.clone();

        let callback = Arc::new(CallbackFn::new(move |_info, painter| {
            renderer.lock().unwrap().render(painter.gl(), self.screen_rect, from, to, 1.0);
        }));

        painter.add(PaintCallback { rect: self.screen_rect, callback});
        
        response
    }
}