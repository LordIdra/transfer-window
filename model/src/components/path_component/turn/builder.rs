use nalgebra_glm::DVec2;

use crate::{components::vessel_component::rcs::Rcs, storage::entity_allocator::Entity};

use super::Turn;

#[derive(Debug, Clone)]
pub struct TurnBuilder {
    pub parent: Entity,
    pub parent_mass: f64,
    pub dry_mass: f64,
    pub fuel_kg: f64,
    pub time: f64,
    pub position: DVec2,
    pub velocity: DVec2,
    pub rotation: f64,
    pub target_rotation: f64,
    pub rcs: Rcs,
}

impl TurnBuilder {
    pub fn build(self) -> Turn {
        Turn::new(self.parent, self.parent_mass, self.dry_mass, self.fuel_kg, self.time, self.position, self.velocity, self.rotation, self.target_rotation, &self.rcs)
    }
}
