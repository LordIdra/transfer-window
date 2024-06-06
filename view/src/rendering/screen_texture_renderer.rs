use std::sync::Arc;

use eframe::{egui::Pos2, glow::{self}};
use glow::Context;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}, texture::Texture};

/// Problem: egui's inbuilt image widgets do not do antialiasing, so
/// the textures look horrible at small scales. Solution: provide
/// our own rendering pipeline which performs MSAA.
pub struct ScreenTextureRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    texture: Texture,
}

impl ScreenTextureRenderer {
    pub fn new(gl: &Arc<Context>, texture: Texture) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/screen_texture.vert"), include_str!("../../resources/shaders/screen_texture.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 }, // position
            VertexAttribute { index: 1, count: 2 }, // texture coordinates
        ]);
        Self { program, vertex_array_object, texture }
    }

    pub fn render(&mut self, from: Pos2, to: Pos2, alpha: f32) {
        let vertices = vec![
            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            to.x, from.y, 1.0, 0.0,

            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            from.x, to.y, 0.0, 1.0,
        ];
        self.vertex_array_object.data(&vertices);
        self.texture.bind();
        self.program.use_program();
        self.program.uniform_float("alpha", alpha);
        self.vertex_array_object.draw();
    }
}