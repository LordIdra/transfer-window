use nalgebra_glm::DVec2;

use crate::storage::entity_allocator::Entity;

use super::{rocket_equation_function::RocketEquationFunction, Burn};

#[derive(Debug, Clone)]
pub struct BurnBuilder {
    pub parent: Entity,
    pub parent_mass: f64,
    pub rocket_equation_function: RocketEquationFunction,
    pub tangent: DVec2,
    pub delta_v: DVec2,
    pub time: f64,
    pub position: DVec2,
    pub velocity: DVec2,
}

impl BurnBuilder {
    pub fn build(self) -> Burn {
        Burn::new(self.parent, self.parent_mass, self.rocket_equation_function, self.tangent, self.delta_v, self.time, self.position, self.velocity)
    }
}