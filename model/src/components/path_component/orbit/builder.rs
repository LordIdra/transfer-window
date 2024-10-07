use nalgebra_glm::{vec2, DVec2};

use crate::{model::{state_query::StateQuery, Model}, storage::entity_allocator::Entity};

use super::{orbit_direction::OrbitDirection, Orbit};

#[derive(Debug, Clone)]
pub struct OrbitBuilder {
    pub parent: Entity,
    pub mass: f64,
    pub parent_mass: f64,
    pub rotation: f64,
    pub position: DVec2,
    pub velocity: DVec2,
    pub time: f64,
}

impl OrbitBuilder {
    pub fn build(self) -> Orbit {
        Orbit::new(self.parent, self.mass, self.parent_mass, self.rotation, self.position, self.velocity, self.time)
    }
}

// For an entity's first orbit
#[derive(Debug, Clone)]
pub enum InitialOrbitBuilder {
    Circular  { parent: Entity, distance: f64, angle: f64, direction: OrbitDirection },
    Freeform  { parent: Entity, distance: f64, angle: f64, direction: OrbitDirection, speed: f64 }
}

impl InitialOrbitBuilder {
    pub fn build(self, model: &Model, mass: f64) -> Orbit {
        match self {
            InitialOrbitBuilder::Circular { parent, distance, angle, direction } => {
                let parent_mass = model.mass(parent);
                let position = distance * vec2(f64::cos(angle), f64::sin(angle));
                Orbit::circle(parent, mass, parent_mass, position, model.time(), direction)
            }
            InitialOrbitBuilder::Freeform { parent, distance, angle, direction, speed } => {
                let parent_mass = model.mass(parent);
                let position = distance * vec2(f64::cos(angle), f64::sin(angle));
                let mut velocity = speed * vec2(f64::sin(angle), -f64::cos(angle));
                if let OrbitDirection::AntiClockwise = direction {
                    velocity *= -1.0;
                }
                let rotation = f64::atan2(velocity.y, velocity.x);
                Orbit::new(parent, mass, parent_mass, rotation, position, velocity, model.time())
            },
        }
    }
}
