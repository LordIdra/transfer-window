use std::sync::Arc;

use eframe::glow::{self};
use glow::{Context, HasContext, TEXTURE_2D, RGBA, UNSIGNED_BYTE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, LINEAR};

#[derive(Clone)]
pub struct ColorBufferTexture {
    gl: Arc<Context>,
    texture: glow::Texture,
}

impl ColorBufferTexture {
    pub fn new(gl: Arc<Context>, screen_size: (i32, i32)) -> Self {
        unsafe {
            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_size.0, screen_size.1, 0, RGBA, UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            ColorBufferTexture { gl, texture }
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }
    }

    pub fn resize(&self, screen_size: (i32, i32)) {
        unsafe { 
            self.bind();
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_size.0, screen_size.1, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub(super) fn texture(&self) -> glow::Texture {
        self.texture
    }
}

impl Drop for ColorBufferTexture {
    fn drop(&mut self) {
        unsafe { 
            self.gl.delete_texture(self.texture);
        };
    }
}