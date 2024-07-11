use std::f64::consts::PI;

use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::View;

use super::ConditionCheck;

pub struct LastOrbitApoapsis {
    entity: Entity,
    min: f64,
    max: f64
}

impl LastOrbitApoapsis {
    pub fn new(entity: Entity, min: f64, max: f64) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity, min, max })
    }
}

impl ConditionCheck for LastOrbitApoapsis {
    fn met(&self, view: &View) -> bool {
        let orbit = &view.model.path_component(self.entity).final_orbit().unwrap();
        let argument_of_apoapsis = orbit.argument_of_periapsis() + PI;
        let apoapsis = orbit.position_from_theta(argument_of_apoapsis).magnitude();
        apoapsis >= self.min && apoapsis <= self.max
    }
}

