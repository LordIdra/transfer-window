use std::sync::Arc;

use eframe::{egui::{Pos2, Rect}, glow::{self, Framebuffer, COLOR_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, LINEAR, READ_FRAMEBUFFER, RGBA, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, UNSIGNED_BYTE}};
use glow::{Context, HasContext};

use super::{shader_program::ShaderProgram, util::{clear_framebuffer, create_multisample_color_attachment, create_normal_color_attachment, SAMPLES}, vertex_array_object::{VertexArrayObject, VertexAttribute}};

/// Problem: egui's inbuilt image widgets do not do antialiasing, so
/// the textures look horrible at small scales. Solution: provide
/// our own rendering pipeline which performs MSAA.
/// Computationally expensive? Yes. Worth it? Fuck yes.
pub struct ScreenTextureRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    multisample_framebuffer: Framebuffer,
    multisample_texture: glow::Texture,
    intermediate_framebuffer: Framebuffer,
    intermediate_texture: glow::Texture,
}

impl ScreenTextureRenderer {
    pub fn new(gl: &Arc<Context>, screen_rect: Rect) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/screen_texture.vert"), include_str!("../../../resources/shaders/screen_texture.frag"));
        let vertex_array_object = VertexArrayObject::new(gl, vec![
            VertexAttribute { index: 0, count: 2 }, // position
            VertexAttribute { index: 1, count: 2 }, // texture coordinates
        ]);
        unsafe {
            let multisample_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let multisample_texture = create_multisample_color_attachment(gl, multisample_framebuffer, screen_rect);

            let intermediate_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let intermediate_texture = create_normal_color_attachment(gl, intermediate_framebuffer, screen_rect);

            Self { program, vertex_array_object, multisample_framebuffer, multisample_texture, intermediate_framebuffer, intermediate_texture }
        }
    }

    pub fn resize(&self, gl: &Arc<Context>, screen_rect: Rect) {
        unsafe {
            gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.multisample_texture));
            gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);

            gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub fn render(&mut self, gl: &Arc<Context>, texture: glow::Texture, screen_rect: Rect, from: Pos2, to: Pos2, alpha: f32) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Screen texture render");
        
        unsafe {
            clear_framebuffer(gl, self.multisample_framebuffer);
            clear_framebuffer(gl, self.intermediate_framebuffer);
        }

        let vertices = vec![
            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            to.x, from.y, 1.0, 0.0,

            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            from.x, to.y, 0.0, 1.0,
        ];
        self.vertex_array_object.data(gl, &vertices);

        unsafe {
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_texture(TEXTURE_2D, Some(texture));
        }

        self.program.use_program(gl);
        self.program.uniform_float(gl, "alpha", alpha);
        self.vertex_array_object.draw(gl);

        unsafe {
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.intermediate_framebuffer));
            let width = screen_rect.width() as i32;
            let height = screen_rect.height() as i32;
            gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, LINEAR);
            gl.bind_framebuffer(FRAMEBUFFER, None);
            gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
        }

        let vertices = vec![
            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, -1.0, 1.0, 0.0,

            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 0.0, 1.0,
        ];
        self.vertex_array_object.data(gl, &vertices);

        self.program.use_program(gl);
        self.program.uniform_float(gl, "alpha", 1.0);
        self.vertex_array_object.draw(gl);
    }

    pub fn destroy(&self, gl: &Arc<Context>) {
        unsafe {
            gl.delete_framebuffer(self.multisample_framebuffer);
            gl.delete_texture(self.multisample_texture);
            gl.delete_framebuffer(self.intermediate_framebuffer);
            gl.delete_texture(self.intermediate_texture);
        }
    }
}