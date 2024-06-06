use std::sync::Arc;

use eframe::glow::{self, LINEAR, LINEAR_MIPMAP_LINEAR};
use glow::{Context, HasContext, TEXTURE_2D, RGBA, UNSIGNED_BYTE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER};

pub struct Texture {
    texture: glow::Texture,
}

impl Texture {
    pub fn new(gl: &Arc<Context>, size: (i32, i32), bytes: &[u8]) -> Self {
        unsafe {
            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(TEXTURE_2D, Some(texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, size.0, size.1, 0, RGBA, UNSIGNED_BYTE, Some(bytes));
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            gl.generate_mipmap(TEXTURE_2D);
            Texture { texture }
        }
    }

    pub fn bind(&self, gl: &Arc<Context>) {
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        unsafe { 
            gl.delete_texture(self.texture);
        };
    }
}