use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;
use crate::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Explosion {
    parent: Entity,
    offset: DVec2,
    combined_mass: f64,
}

impl Explosion {
    pub fn new(parent: Entity, offset: DVec2, combined_mass: f64) -> Self {
        Self {
            parent,
            offset,
            combined_mass,
        }
    }

    pub fn parent(&self) -> Entity {
        self.parent
    }

    pub fn offset(&self) -> DVec2 {
        self.offset
    }

    pub fn combined_mass(&self) -> f64 {
        self.combined_mass
    }
}

impl Model {
    pub(crate) fn add_explosion(&mut self, explosion: Explosion) {
        self.explosions_started_this_frame.push(explosion);
    }

    pub fn explosions_started_this_frame(&self) -> &Vec<Explosion> {
        &self.explosions_started_this_frame
    }
}
