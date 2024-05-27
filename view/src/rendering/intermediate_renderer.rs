use eframe::egui::Rect;

use super::{color_buffer_multisample::ColorBufferMultisample, framebuffer::Framebuffer};

/// Render to the intermediate (multisample) framebuffer
pub struct IntermediateRenderer {
    framebuffer: Framebuffer,
    color_buffer_texture: ColorBufferMultisample,
}

impl IntermediateRenderer {
    pub fn new(framebuffer: Framebuffer, color_buffer_texture: ColorBufferMultisample) -> Self {
        Self { framebuffer, color_buffer_texture }
    }

    pub fn resize(&self, screen_rect: Rect) {
        self.color_buffer_texture.resize(screen_rect);
    }

    pub fn clear(&self) {
        self.framebuffer.clear();
    }

    pub fn set_as_draw_target(&self) {
        self.framebuffer.bind();
    }

    pub fn bind_to_read(&self) {
        self.framebuffer.bind_to_read()
    }
}