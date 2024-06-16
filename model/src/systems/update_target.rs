use crate::{components::ComponentType, Model};

impl Model {
    /// Handles deselecting targets that no longer exist
    pub(crate) fn update_target(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update target");
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            if let Some(target) = self.vessel_component(entity).target() {
                if !self.entity_exists(target) {
                    self.vessel_component_mut(entity).set_target(None);
                }
            }
        }
    }
}