use nalgebra_glm::{vec2, DMat2, DVec2};
use serde::{Deserialize, Serialize};

use crate::components::{path_component::burn::rocket_equation_function::RocketEquationFunction, vessel_component::{system_slot::SlotLocation, VesselClass, VesselComponent}};

#[derive(Debug, Serialize, Deserialize)]
pub struct FireTorpedoEvent {
    slot_location: SlotLocation, 
    rocket_equation_function: RocketEquationFunction,
    tangent: DVec2,
    delta_v: DVec2,
}

impl FireTorpedoEvent {
    pub fn new(slot_location: SlotLocation, velocity: DVec2) -> Self {
        let rocket_equation_function = RocketEquationFunction::from_vessel_component(&VesselComponent::new(VesselClass::Torpedo));
        let tangent = velocity.normalize();
        let delta_v = vec2(0.0, 0.0);
        Self { slot_location, rocket_equation_function, tangent, delta_v }
    }

    pub fn rotation_matrix(&self) -> DMat2 {
        DMat2::new(
            self.tangent.x, -self.tangent.y, 
            self.tangent.y, self.tangent.x)
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.delta_v += adjustment;
    }
}