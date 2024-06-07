use std::sync::Arc;

use eframe::glow;
use glow::Context;
use nalgebra_glm::Mat3;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

pub struct GeometryRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    vertices: Vec<f32>,
}

impl GeometryRenderer {
    pub fn new(gl: &Arc<Context>) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../resources/shaders/geometry.vert"), include_str!("../../resources/shaders/geometry.frag"));
        let vertex_array_object = VertexArrayObject::new(gl, vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // y
            VertexAttribute { index: 2, count: 4 }, // rgba
        ]);
        let vertices = vec![];
        Self { program, vertex_array_object, vertices }
    }

    pub fn add_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.vertices.append(vertices);
    }

    pub fn render(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        self.vertex_array_object.data(gl, &self.vertices);
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw(gl);
        self.vertices.clear();
    }

    pub fn render_lines(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        self.vertex_array_object.data(gl, &self.vertices);
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw_lines(gl);
        self.vertices.clear();
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
    }
}