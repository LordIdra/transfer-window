use std::sync::Arc;

use eframe::glow;
use glow::Context;
use nalgebra_glm::Mat3;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}, texture::Texture};

pub struct TextureRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    texture: Texture,
    vertices: Vec<f32>,
}

impl TextureRenderer {
    pub fn new(gl: Arc<Context>, texture: Texture) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/icon.vert"), include_str!("../../resources/shaders/icon.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // y
            VertexAttribute { index: 2, count: 1 }, // alpha
            VertexAttribute { index: 3, count: 2 }, // texture coordinates
        ]);
        let vertices = vec![];
        Self { program, vertex_array_object, texture, vertices }
    }

    pub fn add_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.vertices.append(vertices);
    }

    pub fn render(&mut self, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        self.vertex_array_object.data(&self.vertices);
        self.texture.bind();
        self.program.use_program();
        self.program.uniform_mat3("zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3("translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3("translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw();
        self.vertices.clear();
    }
}