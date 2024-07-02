use std::f64::consts::TAU;
use crate::components::ComponentType;
use crate::Model;

impl Model {
    pub(crate) fn update_objects(&mut self) {
        let time = self.time;
        for entity in self.entities(vec![ComponentType::OrbitableComponent]) {
            let orbitable_component = self.orbitable_component_mut(entity);
            let rotation_period = orbitable_component.rotation_period_in_secs();
            let rotation_angle = orbitable_component.rotation_angle_mut();
            *rotation_angle = time / rotation_period * TAU;
        }
    }
}