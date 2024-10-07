use nalgebra_glm::DVec2;

use crate::{components::path_component::{orbit::builder::InitialOrbitBuilder, segment::Segment}, model::Model};

use super::OrbitableComponentPhysics;

#[derive(Debug, Clone)]
pub enum OrbitablePhysicsBuilder {
    Stationary(DVec2),
    Orbit(InitialOrbitBuilder),
}

impl OrbitablePhysicsBuilder {
    pub fn build(self, model: &Model, mass: f64) -> OrbitableComponentPhysics {
        match self {
            OrbitablePhysicsBuilder::Stationary(position) => OrbitableComponentPhysics::Stationary(position),
            OrbitablePhysicsBuilder::Orbit(orbit_builder) => OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit_builder.build(model, mass))),
        }
    }
}
