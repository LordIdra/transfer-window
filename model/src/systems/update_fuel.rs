use crate::{components::ComponentType, Model};

impl Model {
    pub(crate) fn update_fuel(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update fuel");
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            if !self.vessel_component(entity).is_ghost() && self.vessel_component(entity).has_fuel_tank() && self.vessel_component(entity).has_engine() {
                let mass = self.mass_at_time(entity, self.time(), None);
                let dry_mass = self.vessel_component(entity).dry_mass();
                self.vessel_component_mut(entity).set_fuel_kg(mass - dry_mass);
            }
        }
    }
}