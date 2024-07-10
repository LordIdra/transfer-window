use std::sync::Arc;

use eframe::glow::{Context, HasContext, Texture, TEXTURE_2D};
use nalgebra_glm::Mat3;

use super::shader_program::ShaderProgram;
use super::vertex_array_object::VertexArrayObject;

pub struct CelestialObjectRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    texture: Texture,
    clouds: Vec<Texture>,
    vertices: Vec<f32>,
    rotation: f32,
    cloud_speeds: Vec<f64>,
}

impl CelestialObjectRenderer {
    pub fn new(gl: &Arc<Context>, texture: Texture, clouds: Vec<Texture>) -> Self {
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/celestial_object.vert"), include_str!("../../../resources/shaders/celestial_object.frag"));
        let vertex_array_object = VertexArrayObject::texture_vertex_array(gl);
        let vertices = vec![];
        let rotation = 0.0;
        let cloud_speeds = vec![];
        Self { program, vertex_array_object, texture, clouds, vertices, rotation, cloud_speeds }
    }

    pub fn add_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.vertices.append(vertices);
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    
    pub fn set_cloud_speeds(&mut self, cloud_speeds: Vec<f64>) {
        self.cloud_speeds = cloud_speeds;
    }

    pub fn render(&mut self, gl: &Arc<Context>, zoom_matrix: Mat3, translation_matrices: (Mat3, Mat3)) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Planet render");
        assert_eq!(self.clouds.len(), self.cloud_speeds.len(), "Clouds must have corresponding speeds");
        self.render_layer(gl, zoom_matrix, translation_matrices, self.texture, self.rotation);
        for (i, cloud) in self.clouds.clone().iter().enumerate() {
            let speed = self.cloud_speeds[i] as f32;
            let rotation = self.rotation * speed;
            self.render_layer(gl, zoom_matrix, translation_matrices, *cloud, rotation);
        }
        self.vertices.clear();
    }

    fn render_layer(
        &mut self,
        gl: &Arc<Context>,
        zoom_matrix: Mat3,
        translation_matrices: (Mat3, Mat3),
        texture: Texture,
        rotation: f32,
    ) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Planet layer render");
        self.vertex_array_object.data(gl, &self.vertices);
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(texture));
        }
        self.program.use_program(gl);
        self.program.uniform_mat3(gl, "zoom_matrix", zoom_matrix.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3(gl, "translation_matrix_lower", translation_matrices.1.as_slice());
        self.program.uniform_float(gl, "rotation", rotation);
        self.vertex_array_object.draw(gl);
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
    }
}