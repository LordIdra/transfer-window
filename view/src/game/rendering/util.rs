use std::sync::Arc;

use eframe::{egui::Rect, glow::{Context, Framebuffer, HasContext, Texture, CLAMP_TO_EDGE, COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, FRAMEBUFFER, NEAREST, RGBA, RGBA8, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNSIGNED_BYTE}};

pub const SAMPLES: i32 = 4;

pub unsafe fn create_multisample_color_attachment(gl: &Arc<Context>, framebuffer: Framebuffer, screen_rect: Rect) -> Texture {
    unsafe {
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
        gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(texture));
        gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
        gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D_MULTISAMPLE, Some(texture), 0);
        texture
    }
}

pub unsafe fn create_normal_color_attachment(gl: &Arc<Context>, framebuffer: Framebuffer, screen_rect: Rect) -> Texture {
    unsafe {
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
        gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(texture), 0);
        texture
    }
}

pub unsafe fn clear_framebuffer(gl: &Arc<Context>, framebuffer: Framebuffer) {
    gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(COLOR_BUFFER_BIT);

}
