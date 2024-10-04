use std::sync::{Arc, Mutex};

use eframe::{egui::{Color32, Context, CursorIcon, PaintCallback, Pos2, Rect, Response, Sense, Ui, Vec2, Widget}, egui_glow::CallbackFn, emath::RectTransform, glow};

use crate::game::{camera::Camera, rendering::screen_texture_renderer::ScreenTextureRenderer, View};

const NORMAL_ALPHA: f32 = 1.0;
const DISABLED_ALPHA: f32 = 0.4;
const HOVERED_CIRCLE_ALPHA: u8 = 40;

pub struct CustomCircularImageButton {
    context: Context,
    renderer: Arc<Mutex<ScreenTextureRenderer>>,
    texture: glow::Texture,
    screen_rect: Rect,
    size: i32,
    sense: Sense,
    enabled: bool,
    normal_color: Color32,
    hover_color: Color32,
    pointer: bool,
}

impl CustomCircularImageButton {
    pub fn new(view: &View, texture_name: &str, size: i32) -> Self {
        let context = view.context.clone();
        let renderer = view.renderers.screen_texture_renderer();
        let texture = view.resources.gl_texture(texture_name);
        let screen_rect = view.screen_rect;
        let sense = Sense::union(Sense::click(), Sense::hover());
        let enabled = true;
        let normal_color = Color32::TRANSPARENT;
        let hover_color = Color32::from_rgb(HOVERED_CIRCLE_ALPHA, HOVERED_CIRCLE_ALPHA, HOVERED_CIRCLE_ALPHA);
        let pointer = true;
        Self { context, renderer, texture, screen_rect, size, sense, enabled, normal_color, hover_color, pointer }
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

        let (response, painter) = ui.allocate_painter(Vec2::splat(self.size as f32), self.sense);
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let alpha = if self.enabled {
            NORMAL_ALPHA
        } else {
            DISABLED_ALPHA
        };

        let center = to_screen.transform_pos(Pos2::new(self.size as f32 / 2.0, self.size as f32 / 2.0));
        let radius = self.size as f32 / 2.0;
        let circle_color = if response.hovered() || response.clicked() {
            if self.pointer {
                self.context.set_cursor_icon(CursorIcon::PointingHand);
            }
            self.hover_color
        } else {
            self.normal_color
        };
        painter.circle_filled(center, radius, circle_color);

        let corner = Camera::window_space_to_screen_space(self.screen_rect, to_screen.transform_pos(Pos2::new(0.0, 0.0)));
        let renderer = self.renderer.clone();

        let callback = Arc::new(CallbackFn::new(move |_info, painter| {
            renderer.lock().unwrap().render(painter.gl(), self.texture, self.screen_rect, corner, self.size, self.size, alpha);
        }));

        painter.add(PaintCallback { rect: self.screen_rect, callback});
        
        response
    }
}
