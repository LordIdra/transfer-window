use nalgebra_glm::DVec2;

use crate::{components::vessel_component::engine::Engine, storage::entity_allocator::Entity};

use super::Burn;

#[derive(Debug, Clone)]
pub struct BurnBuilder {
    pub parent: Entity,
    pub parent_mass: f64,
    pub mass: f64,
    pub fuel_mass: f64,
    pub engine: Engine,
    pub tangent: DVec2,
    pub delta_v: DVec2,
    pub time: f64,
    pub position: DVec2,
    pub velocity: DVec2,
}

impl BurnBuilder {
    pub fn build(self) -> Burn {
        Burn::new(self.parent, self.parent_mass, self.mass, self.fuel_mass, &self.engine, self.tangent, self.delta_v, self.time, self.position, self.velocity)
    }
}
