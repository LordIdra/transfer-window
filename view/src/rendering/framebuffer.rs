use std::sync::Arc;

use eframe::glow::{self, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, READ_FRAMEBUFFER};
use glow::{Context, HasContext};

pub struct Framebuffer {
    gl: Arc<Context>,
    framebuffer: glow::Framebuffer,
}

impl Framebuffer {
    pub fn new(gl: Arc<Context>) -> Self {
        unsafe {
            let framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            Framebuffer { gl, framebuffer }
        }
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

    pub fn bind_to_read(&self) {
        unsafe {
            self.gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.framebuffer));
        }
    }

    pub fn bind_to_draw(&self) {
        unsafe {
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.framebuffer));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }
}