use crate::{components::ComponentType, Model};

impl Model {
    pub(crate) fn update_launcher_cooldown(&mut self, dt: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update launcher cooldown");
        let time_step = self.time_step().time_step();
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            if !self.vessel_component(entity).is_ghost() && self.vessel_component(entity).has_torpedo_launcher() {
                self.vessel_component_mut(entity).step_torpedo_launcher(dt * time_step);
            }
        }
    }
}