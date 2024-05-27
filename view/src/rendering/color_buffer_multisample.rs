use std::sync::Arc;

use eframe::{egui::Rect, glow::{self, COLOR_ATTACHMENT0, FRAMEBUFFER, TEXTURE_2D_MULTISAMPLE}};
use glow::{Context, HasContext, RGBA};

use super::framebuffer::Framebuffer;

pub struct ColorBufferMultisample {
    gl: Arc<Context>,
    texture: glow::Texture,
}

impl ColorBufferMultisample {
    pub fn new(gl: Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(texture));
            gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, 4, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
            ColorBufferMultisample { gl, texture }
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.texture));
        }
    }

    pub fn resize(&self, screen_rect: Rect) {
        unsafe { 
            self.bind();
            self.gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, 16, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
        }
    }

    pub fn attach_to_framebuffer(&self, framebuffer: &Framebuffer) {
        unsafe {
            framebuffer.bind();
            self.bind();
            self.gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D_MULTISAMPLE, Some(self.texture), 0);
            framebuffer.unbind();
        }
    }
}

impl Drop for ColorBufferMultisample {
    fn drop(&mut self) {
        unsafe { 
            self.gl.delete_texture(self.texture);
        };
    }
}