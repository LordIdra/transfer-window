use std::sync::{Arc, Mutex};

use eframe::egui::{
    Color32, Context, CursorIcon, PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget,
};
use eframe::egui_glow::CallbackFn;
use eframe::emath::RectTransform;
use eframe::glow;

use crate::game::camera::Camera;
use crate::game::rendering::screen_texture_renderer::ScreenTextureRenderer;
use crate::game::View;

const NORMAL_ALPHA: f32 = 1.0;
const DISABLED_ALPHA: f32 = 0.4;
const HOVERED_CIRCLE_ALPHA: u8 = 40;

pub struct CustomCircularImageButton {
    context: Context,
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    texture: glow::Texture,
    screen_rect: Rect,
    size: f32,
    sense: Sense,
    padding: f32,
    margin: f32,
    enabled: bool,
    normal_color: Color32,
    hover_color: Color32,
    pointer: bool,
}

impl CustomCircularImageButton {
    pub fn new(view: &View, texture_name: &str, size: f32) -> Self {
        let context = view.context.clone();
        let renderer = view.renderers.screen_texture_renderer();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let padding = 0.0;
        let margin = 0.0;
        let enabled = true;
        let normal_color = Color32::TRANSPARENT;
        let hover_color = Color32::from_rgb(
            HOVERED_CIRCLE_ALPHA,
            HOVERED_CIRCLE_ALPHA,
            HOVERED_CIRCLE_ALPHA,
        );
        let pointer = true;
        Self {
            context,
            renderer,
            texture,
            screen_rect,
            size,
            sense,
            padding,
            margin,
            enabled,
            normal_color,
            hover_color,
            pointer,
        }
    }

    /// Space around the icon, not including circle
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Space around the entire widget including circle
    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_normal_color(mut self, color: Color32) -> Self {
        self.normal_color = color;
        self
    }

    pub fn with_hover_color(mut self, color: Color32) -> Self {
        self.hover_color = color;
        self
    }

    pub fn with_pointer(mut self, pointer: bool) -> Self {
        self.pointer = pointer;
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

        let center = to_screen.transform_pos(Pos2::new(self.size / 2.0, self.size / 2.0));
        let radius = self.size / 2.0 - self.margin;
        let circle_color = if response.hovered() || response.clicked() {
            if self.pointer {
                self.context.set_cursor_icon(CursorIcon::PointingHand);
            }
            self.hover_color
        } else {
            self.normal_color
        };
        painter.circle_filled(center, radius, circle_color);

        let total_padding = self.margin + self.padding;
        let from = Camera::window_space_to_screen_space(
            self.screen_rect,
            to_screen.transform_pos(Pos2::new(total_padding, total_padding)),
        );
        let to = Camera::window_space_to_screen_space(
            self.screen_rect,
            to_screen.transform_pos(Pos2::new(
                self.size - total_padding,
                self.size - total_padding,
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
                alpha,
            );
        }));

        painter.add(PaintCallback {
            rect: self.screen_rect,
            callback,
        });

        response
    }
}
