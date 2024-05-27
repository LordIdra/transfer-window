use std::sync::Arc;

use eframe::{egui::Rect, glow::{self, COLOR_BUFFER_BIT, NEAREST}};
use glow::{Context, HasContext};

use super::{color_buffer_normal::ColorBufferNormal, framebuffer::Framebuffer, intermediate_renderer::IntermediateRenderer, shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

pub struct ScreenRenderer {
    gl: Arc<Context>,
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    framebuffer: Framebuffer,
    color_buffer_texture: ColorBufferNormal,
}

impl ScreenRenderer {
    pub fn new(gl: Arc<Context>, framebuffer: Framebuffer, color_buffer_texture: ColorBufferNormal) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/screen.vert"), include_str!("../../resources/shaders/screen.frag"));
        let mut vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // texture coordinates
        ]);
        let vertices = vec![
            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, -1.0, 1.0, 0.0,

            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 0.0, 1.0,
        ];
        vertex_array_object.data(&vertices);
        Self { gl, program, vertex_array_object, framebuffer, color_buffer_texture }
    }

    pub fn resize(&self, screen_rect: Rect) {
        self.color_buffer_texture.resize(screen_rect);
    }

    pub fn render(&self) {
        self.color_buffer_texture.bind();
        self.program.use_program();
        self.vertex_array_object.draw();
    }

    pub fn clear(&self) {
        self.framebuffer.clear();
    }

    pub fn set_screen_as_draw_target(&self) {
        self.framebuffer.unbind();
    }

    pub fn blit_from(&self, intermediate_renderer: &IntermediateRenderer, screen_rect: Rect) {
        intermediate_renderer.bind_to_read();
        self.framebuffer.bind_to_draw();
        unsafe {
            let width = screen_rect.width() as i32;
            let height = screen_rect.height() as i32;
            self.gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);
        }
    }
}