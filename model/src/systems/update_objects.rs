use std::f64::consts::TAU;

use crate::components::ComponentType;
use crate::Model;

impl Model {
    pub(crate) fn update_objects(&mut self) {
        for entity in self.entities(vec![ComponentType::OrbitableComponent]) {
            let rotation_period = self.orbitable_component(entity).rotation_period_in_secs();
            let rotation_angle = self.time / rotation_period * TAU;
            self.orbitable_component_mut(entity).set_rotation_angle(rotation_angle);
        }
    }
}
