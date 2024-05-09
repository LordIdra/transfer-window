use crate::{components::ComponentType, Model};

impl Model {
    pub fn update_fuel(&mut self) {
        let time = self.time();
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            if !self.vessel_component(entity).is_ghost() {
                let mass = self.mass_at_time(entity, time);
                let dry_mass = self.vessel_component(entity).dry_mass();
                self.vessel_component_mut(entity).slots_mut().set_fuel_kg(mass - dry_mass);
            }
        }
    }
}