use std::sync::Arc;

use eframe::glow;
use glow::Context;

use super::{color_buffer_texture::ColorBufferTexture, shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

pub struct ScreenRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    color_buffer_texture: ColorBufferTexture,
}

impl ScreenRenderer {
    pub fn new(gl: Arc<Context>, color_buffer_texture: ColorBufferTexture) -> Self {
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
        Self { program, vertex_array_object, color_buffer_texture }
    }

    pub fn render(&mut self) {
        self.color_buffer_texture.bind();
        self.program.use_program();
        self.vertex_array_object.draw();
    }
}