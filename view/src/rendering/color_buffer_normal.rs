use std::sync::Arc;

use eframe::{egui::Rect, glow::{self, COLOR_ATTACHMENT0, FRAMEBUFFER}};
use glow::{Context, HasContext, TEXTURE_2D, RGBA, UNSIGNED_BYTE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, LINEAR};

use super::framebuffer::Framebuffer;

pub struct ColorBufferNormal {
    gl: Arc<Context>,
    texture: glow::Texture,
}

impl ColorBufferNormal {
    pub fn new(gl: Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            ColorBufferNormal { gl, texture }
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }
    }

    pub fn resize(&self, screen_rect: Rect) {
        unsafe { 
            self.bind();
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub fn attach_to_framebuffer(&self, framebuffer: &Framebuffer) {
        unsafe {
            framebuffer.bind();
            self.bind();
            self.gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(self.texture), 0);
            framebuffer.unbind();
        }
    }
}

impl Drop for ColorBufferNormal {
    fn drop(&mut self) {
        unsafe { 
            self.gl.delete_texture(self.texture);
        };
    }
}