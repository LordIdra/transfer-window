use std::sync::{Arc, Mutex};

use eframe::{egui::{Color32, PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget}, egui_glow::CallbackFn, emath::RectTransform, glow};

use crate::game::{camera::Camera, rendering::screen_texture_renderer::ScreenTextureRenderer, View};

const NORMAL_ALPHA: f32 = 1.0;
const DISABLED_ALPHA: f32 = 0.4;
const HOVERED_CIRCLE_ALPHA: u8 = 40;

pub struct CustomCircularImageButton {
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    texture: glow::Texture,
    screen_rect: Rect,
    size: f32,
    sense: Sense,
    padding: f32,
    enabled: bool,
}

impl CustomCircularImageButton {
    pub fn new(view: &View, texture_name: &str, size: f32) -> Self {
        let renderer = view.renderers.screen_texture_renderer();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let padding = 0.0;
        let enabled = true;
        Self { renderer, texture, screen_rect, size, sense, padding, enabled }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for CustomCircularImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Draw custom image button");

        let (response, painter) = ui.allocate_painter(Vec2::splat(self.size), self.sense);
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let alpha = if self.enabled {
            NORMAL_ALPHA
        } else {
            DISABLED_ALPHA
        };

        if response.hovered() || response.clicked() {
            let center = to_screen.transform_pos(Pos2::new(self.size / 2.0, self.size / 2.0));
            let radius = self.size / 2.0;
            let fill = Color32::from_rgb(HOVERED_CIRCLE_ALPHA, HOVERED_CIRCLE_ALPHA, HOVERED_CIRCLE_ALPHA);
            painter.circle_filled(center, radius, fill);
        }

        let from = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(self.padding, self.padding)));
        let to = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(self.size - self.padding, self.size - self.padding)));
        let renderer = self.renderer.clone();

        let callback = Arc::new(CallbackFn::new(move |_info, painter| {
            renderer.lock().unwrap().render(painter.gl(), self.texture, self.screen_rect, from, to, alpha);
        }));

        painter.add(PaintCallback { rect: self.screen_rect, callback});
        
        response
    }
}