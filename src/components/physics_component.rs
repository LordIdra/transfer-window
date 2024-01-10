use nalgebra_glm::{DVec2, vec2};

use crate::storage::entity_allocator::Entity;

use self::trajectory_type::PhysicsType;

mod trajectory;
mod trajectory_type;

pub struct PhysicsComponent {
    mass: f64,
    physics_type: PhysicsType,
}

impl PhysicsComponent {
    pub fn new(mass: f64, physics_type: PhysicsType) -> Self {
        Self { mass, physics_type }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_position(&self) -> DVec2 {
        match self.physics_type {
            PhysicsType::Stationary(position) => position,
            PhysicsType::Trajectory(_) => todo!(),
        }
    }

    pub fn get_velocity(&self) -> DVec2 {
        match self.physics_type {
            PhysicsType::Stationary(_) => vec2(0.0, 0.0),
            PhysicsType::Trajectory(_) => todo!(),
        }
    }

    pub fn get_parent(&self) -> Option<Entity> {
        match self.physics_type {
            PhysicsType::Stationary(_) => None,
            PhysicsType::Trajectory(_) => todo!(),
        }
    }
}