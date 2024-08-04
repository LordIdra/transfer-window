use crate::{components::ComponentType, Model};

impl Model {
    /// Handles deselecting targets that no longer exist or have docked
    pub(crate) fn update_target(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update target");
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            if let Some(target) = self.vessel_component(entity).target() {
                let docked = self.try_path_component(target).is_none() && self.try_vessel_component(target).is_some();
                if !self.entity_exists(target) || docked {
                    self.vessel_component_mut(entity).set_target(None);
                }
            }
        }
    }
}
