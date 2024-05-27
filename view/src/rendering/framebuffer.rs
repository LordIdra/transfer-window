use std::sync::Arc;

use eframe::{egui::Rect, glow::{self, COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, FRAMEBUFFER, FRAMEBUFFER_COMPLETE, TEXTURE_2D}};
use glow::{Context, HasContext};

use super::color_buffer_texture::ColorBufferTexture;

#[derive(Clone)]
pub struct Framebuffer {
    gl: Arc<Context>,
    framebuffer: glow::Framebuffer,
    color_buffer_texture: ColorBufferTexture,
}

impl Framebuffer {
    pub fn new(gl: Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            let framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let color_buffer_texture = ColorBufferTexture::new(gl.clone(), (screen_rect.width() as i32, screen_rect.height() as i32));

            gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
            color_buffer_texture.bind();
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(color_buffer_texture.texture()), 0);
            gl.bind_framebuffer(FRAMEBUFFER, None);

            if gl.check_framebuffer_status(FRAMEBUFFER) != FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer incomplete");
            }

            Framebuffer { gl, framebuffer, color_buffer_texture }
        }
    }

    pub fn texture(&self) -> ColorBufferTexture {
        self.color_buffer_texture.clone()
    }

    pub fn clear(&self) {
        self.bind();
        unsafe {
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.framebuffer));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }

    /// When the screen is resized, the FBO will also need resizing, so we just regenerate it
    pub fn regenerate(&mut self, screen_rect: Rect) {
        self.color_buffer_texture.resize((screen_rect.width() as i32, screen_rect.height() as i32));
    }
}