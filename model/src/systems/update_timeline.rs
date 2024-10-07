use log::debug;

use crate::{components::ComponentType, model::Model, storage::entity_allocator::Entity};

impl Model {
    fn update_entity_timeline(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update entity timeline");
        let time = self.time();
        let Some(vessel_component) = self.try_vessel_component_mut(entity) else {
            // Vessel was destroyed in a previously executed event
            return;
        };
        let events = vessel_component.timeline_mut().pop_events_before(time);
        for event in events {
            debug!("Executing event {event:?}");
            event.execute(self);
        }
    }

    pub(crate) fn update_timeline(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update timeline");
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            self.update_entity_timeline(entity);
        }
    }
}
