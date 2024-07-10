use std::sync::{Arc, Mutex};

use eframe::egui::{PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget};
use eframe::egui_glow::CallbackFn;
use eframe::emath::RectTransform;
use eframe::glow;

use crate::game::camera::Camera;
use crate::game::rendering::screen_texture_renderer::ScreenTextureRenderer;
use crate::game::View;

pub struct CustomImage {
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    texture: glow::Texture,
    screen_rect: Rect,
    width: f32,
    height: f32,
    sense: Sense,
    padding: f32,
    alpha: f32,
}

impl CustomImage {
    pub fn new(view: &View, texture_name: &str, size: f32) -> Self {
        let renderer = view.renderers.screen_texture_renderer();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let width = size;
        let height = size;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let padding = 0.0;
        let alpha = 1.0;
        Self {
            renderer,
            texture,
            screen_rect,
            width,
            height,
            sense,
            padding,
            alpha,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_from_parts(
        renderer: Arc<Mutex<ScreenTextureRenderer>>,
        texture: glow::Texture,
        screen_rect: Rect,
        width: f32,
        height: f32,
        sense: Sense,
        padding: f32,
        alpha: f32,
    ) -> Self {
        Self {
            renderer,
            texture,
            screen_rect,
            width,
            height,
            sense,
            padding,
            alpha,
        }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
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

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), self.sense);
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let from = Camera::window_space_to_screen_space(
            self.screen_rect,
            to_screen.transform_pos(Pos2::new(self.padding, self.padding)),
        );
        let to = Camera::window_space_to_screen_space(
            self.screen_rect,
            to_screen.transform_pos(Pos2::new(
                self.width - self.padding,
                self.height - self.padding,
            )),
        );
        let renderer = self.renderer.clone();

        let callback = Arc::new(CallbackFn::new(move |_info, painter| {
            renderer.lock().unwrap().render(
                painter.gl(),
                self.texture,
                self.screen_rect,
                from,
                to,
                self.alpha,
            );
        }));

        painter.add(PaintCallback {
            rect: self.screen_rect,
            callback,
        });

        response
    }
}
