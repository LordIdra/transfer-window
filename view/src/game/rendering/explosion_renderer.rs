use std::sync::Arc;

use eframe::{egui::{Pos2, Rect}, glow};
use glow::Context;
use nalgebra_glm::DVec2;
use transfer_window_model::{model::state_query::StateQuery, storage::entity_allocator::Entity};

use crate::game::View;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

pub struct ExplosionRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    explosion_time: f64,
    duration: f64,
    parent: Entity,
    offset: DVec2,
    size: f64,
    speed: f64,
    center: Option<Pos2>,
}

impl ExplosionRenderer {
    pub fn new(gl: &Arc<Context>, explosion_time: f64, parent: Entity, offset: DVec2, combined_mass: f64) -> Self {
        let size = combined_mass / 2.0e5; 
        let speed = 1.0e6 / combined_mass;
        let duration = 5.0e3 / speed;
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/explosion.vert"), include_str!("../../../resources/shaders/explosion.frag"));
        let mut vertex_array_object = VertexArrayObject::new(gl, vec![
            VertexAttribute { index: 0, count: 2 },
        ]);
        let vertices = vec![
            -1.0, -1.0,
            1.0, 1.0,
            1.0, -1.0,

            -1.0, -1.0,
            1.0, 1.0,
            -1.0, 1.0,
        ];
        vertex_array_object.data(gl, &vertices);
        let center = None;
        Self { program, vertex_array_object, explosion_time, duration, parent, offset, size, speed, center }
    }

    pub fn update_position(&mut self, view: &View) {
        let world_coords = view.model.absolute_position(self.parent) + self.offset;
        self.center = Some(view.world_space_to_window_space(world_coords));
    }

    pub fn is_finished(&self, time: f64) -> bool {
        let time_since_start = time - self.explosion_time;
        time_since_start > self.duration
    }

    pub fn render(&self, gl: &Arc<Context>, time: f64, screen_rect: Rect, zoom: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Explosion render");
        let time_since_start = (time - self.explosion_time) as f32;
        let center = self.center.unwrap();
        
        self.program.use_program(gl);
        self.program.uniform_float(gl, "time", time_since_start);
        self.program.uniform_float(gl, "width", screen_rect.width());
        self.program.uniform_float(gl, "height", screen_rect.height());
        self.program.uniform_float(gl, "zoom", zoom as f32);
        self.program.uniform_vec2(gl, "center", center.x, center.y);
        self.program.uniform_float(gl, "size", self.size as f32);
        self.program.uniform_float(gl, "speed", self.speed as f32);
        self.vertex_array_object.draw(gl);
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        self.program.destroy(gl);
        self.vertex_array_object.destroy(gl);
    }
}
