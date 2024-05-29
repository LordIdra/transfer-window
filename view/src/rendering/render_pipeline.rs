use std::sync::Arc;

use eframe::{egui::Rect, glow::{Context, Framebuffer, HasContext, Texture, COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, LINEAR, NEAREST, READ_FRAMEBUFFER, RGBA, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNSIGNED_BYTE}};

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

const SAMPLES: i32 = 16;

pub struct RenderPipeline {
    gl: Arc<Context>,
    multisample_framebuffer: Framebuffer,
    multisample_texture: Texture,
    intermediate_framebuffer: Framebuffer,
    intermediate_texture: Texture,
    screen_program: ShaderProgram,
    screen_vao: VertexArrayObject,
}

impl RenderPipeline {
    pub fn new(gl: Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            // Generate multisampled framebuffer with texture color attachment
            let multisample_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let multisample_texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_framebuffer(FRAMEBUFFER, Some(multisample_framebuffer));
            gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(multisample_texture));
            gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D_MULTISAMPLE, Some(multisample_texture), 0);

            // Generate intermediate framebuffer with texture color attachment
            let intermediate_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let intermediate_texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_framebuffer(FRAMEBUFFER, Some(intermediate_framebuffer));
            gl.bind_texture(TEXTURE_2D, Some(intermediate_texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(intermediate_texture), 0);
            gl.bind_framebuffer(FRAMEBUFFER, None);

            // Generate program/VAO for rendering final textures to the screen
            let screen_program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/screen.vert"), include_str!("../../resources/shaders/screen.frag"));
            let mut screen_vao = VertexArrayObject::new(gl.clone(), vec![
                VertexAttribute { index: 0, count: 2 }, // x
                VertexAttribute { index: 1, count: 2 }, // texture coordinates
            ]);
            let screen_vertices = vec![
                -1.0, -1.0, 0.0, 0.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, -1.0, 1.0, 0.0,

                -1.0, -1.0, 0.0, 0.0,
                1.0, 1.0, 1.0, 1.0,
                -1.0, 1.0, 0.0, 1.0,
            ];
            screen_vao.data(&screen_vertices);

            Self { gl, multisample_framebuffer, multisample_texture, intermediate_framebuffer, intermediate_texture, screen_program, screen_vao }
        }
    }

    pub fn resize(&self, screen_rect: Rect) {
        unsafe {
            // Resize multisample texture
            self.gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.intermediate_texture));
            self.gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);

            // Resize intermediate texture
            self.gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub fn render(&self, render_bloom: impl FnOnce(), render_normal: impl FnOnce(), screen_rect: Rect) {
        let width = screen_rect.width() as i32;
        let height = screen_rect.height() as i32;

        unsafe {
            // Clear multisample framebuffer
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(COLOR_BUFFER_BIT);

            // Clear intermediate framebuffer
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.intermediate_framebuffer));
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(COLOR_BUFFER_BIT);

            // Render to multisample framebuffer
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            render_bloom();
            render_normal();

            // Blit from multisample framebuffer to intermediate framebuffer
            self.gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.intermediate_framebuffer));
            self.gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);

            // Render final texture to screen
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
            self.gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            self.screen_program.use_program();
            self.screen_vao.draw();
        }
    }
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_framebuffer(self.multisample_framebuffer);
            self.gl.delete_texture(self.multisample_texture);
            self.gl.delete_framebuffer(self.intermediate_framebuffer);
            self.gl.delete_texture(self.intermediate_texture);
        }
    }
}