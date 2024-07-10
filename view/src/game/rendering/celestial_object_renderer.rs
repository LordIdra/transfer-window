use std::sync::Arc;

use eframe::glow;
use eframe::glow::{Context, HasContext, TEXTURE_2D};
use nalgebra_glm::Mat3;

use super::shader_program::ShaderProgram;
use super::vertex_array_object::VertexArrayObject;

pub struct CelestialObjectRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    texture: glow::Texture,
    vertices: Vec<f32>,
    rotation: f32
}

impl CelestialObjectRenderer {
    pub fn new(gl: &Arc<Context>, texture: glow::Texture) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/celestial_object.vert"), include_str!("../../../resources/shaders/celestial_object.frag"));
        let vertex_array_object = VertexArrayObject::texture_vertex_array(gl);
        let vertices = vec![];
        let rotation = 0.0;
        Self { program, vertex_array_object, texture, vertices, rotation }
    }
    
    pub fn add_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.vertices.append(vertices);
    }
    
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn render(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Planet render");
        self.vertex_array_object.data(gl, &self.vertices);
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.program.uniform_float(gl, "rotation", self.rotation);
        self.vertex_array_object.draw(gl);
        self.vertices.clear();
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
    }
}