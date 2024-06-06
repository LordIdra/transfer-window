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
    pub fn new(gl: &Arc<Context>, texture: Texture) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../resources/shaders/icon.vert"), include_str!("../../resources/shaders/icon.frag"));
        let vertex_array_object = VertexArrayObject::new(gl, vec![
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

    pub fn render(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        self.vertex_array_object.data(gl, &self.vertices);
        self.texture.bind(gl);
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw(gl);
        self.vertices.clear();
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
        self.texture.destroy(gl);
    }
}