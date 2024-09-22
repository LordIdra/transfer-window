use nalgebra_glm::DVec2;

use crate::{components::vessel_component::{engine::Engine, faction::Faction}, storage::entity_allocator::Entity, Model};

use super::Guidance;

#[derive(Debug, Clone)]
pub struct GuidanceBuilder {
    pub parent: Entity,
    pub parent_mass: f64,
    pub target: Entity,
    pub faction: Faction,
    pub engine: Engine,
    pub mass: f64,
    pub fuel_kg: f64,
    pub time: f64,
    pub rotation: f64,
    pub position: DVec2,
    pub velocity: DVec2,
}

impl GuidanceBuilder {
    pub fn build(self, model: &Model) -> Guidance {
        Guidance::new(model, self.parent, self.parent_mass, self.target, self.faction, &self.engine, self.mass, self.fuel_kg, self.time, self.rotation, self.position, self.velocity)
    }
}
