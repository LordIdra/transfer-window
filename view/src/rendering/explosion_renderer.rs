use std::sync::Arc;

use eframe::{egui::{Pos2, Rect}, glow};
use glow::Context;
use nalgebra_glm::DVec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::camera::Camera;

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
    pub fn new(gl: Arc<Context>, explosion_time: f64, parent: Entity, offset: DVec2, combined_mass: f64) -> Self {
        let size = combined_mass / 2.0e5; 
        let speed = 1.0e6 / combined_mass;
        let duration = 5.0e3 / speed;
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/explosion.vert"), include_str!("../../resources/shaders/explosion.frag"));
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
        vertex_array_object.data(&vertices);
        let center = None;
        Self { program, vertex_array_object, explosion_time, duration, parent, offset, size, speed, center }
    }

    pub fn update_position(&mut self, camera: &mut Camera, model: &Model, screen_rect: Rect) {
        let world_coords = model.absolute_position(self.parent) + self.offset;
        self.center = Some(camera.world_space_to_window_space(model, world_coords, screen_rect));
    }

    pub fn is_finished(&self, time: f64) -> bool {
        let time_since_start = time - self.explosion_time;
        time_since_start > self.duration
    }

    pub fn render(&self, time: f64, screen_rect: Rect, zoom: f64) {
        let time_since_start = (time - self.explosion_time) as f32;
        let center = self.center.unwrap();
        
        self.program.use_program();
        self.program.uniform_float("time", time_since_start);
        self.program.uniform_float("width", screen_rect.width());
        self.program.uniform_float("height", screen_rect.height());
        self.program.uniform_float("zoom", zoom as f32);
        self.program.uniform_vec2("center", center.x, center.y);
        self.program.uniform_float("size", self.size as f32);
        self.program.uniform_float("speed", self.speed as f32);
        self.vertex_array_object.draw();
    }
}