use std::sync::Arc;

use eframe::epaint::Rgba;
use eframe::glow::Context;
use nalgebra_glm::Mat3;
use transfer_window_model::components::atmosphere_component::AtmosphereComponent;
use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

pub struct AtmosphereRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    vertices: Vec<f32>,
    height: f32,
    color: Rgba
}

impl AtmosphereRenderer {
    pub fn new(gl: &Arc<Context>, atmosphere: &AtmosphereComponent) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/atmosphere.vert"), include_str!("../../../resources/shaders/atmosphere.frag"));
        let vertex_array_object = VertexArrayObject::new(gl, vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // y
            VertexAttribute { index: 2, count: 1 }, // alpha
            VertexAttribute { index: 3, count: 2 }, // uv
        ]);
        let vertices = vec![];
        let height = atmosphere.height() as f32;
        let atmo_color = atmosphere.color();
        let color = Rgba::from_rgba_premultiplied(
            atmo_color.r(),
            atmo_color.g(),
            atmo_color.b(),
            atmosphere.density() as f32
        );
        Self { program, vertex_array_object, vertices, height, color }
    }
    
    pub fn add_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.vertices.append(vertices);
    }

    pub fn render(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Atmosphere render");
        self.vertex_array_object.data(gl, &self.vertices);
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.program.uniform_float(gl, "height", self.height);
        self.program.uniform_vec4(
            gl,
            "color",
            self.color.r(),
            self.color.g(),
            self.color.b(),
            self.color.a()
        );
        self.vertex_array_object.draw(gl);
        self.vertices.clear();
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
    }
}