use log::debug;

use crate::components::ComponentType;
use crate::storage::entity_allocator::Entity;
use crate::Model;

fn update_entity_timeline(model: &mut Model, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update entity timeline");
    let time = model.time();
    let Some(vessel_component) = model.try_vessel_component_mut(entity) else {
        // Vessel was destroyed in a previously executed event
        return;
    };
    let events = vessel_component.timeline_mut().pop_events_before(time);
    for event in events {
        debug!("Executing event {event:?}");
        event.execute(model);
    }
}

impl Model {
    pub(crate) fn update_timeline(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update timeline");
        for entity in self.entities(vec![
            ComponentType::VesselComponent,
            ComponentType::PathComponent,
        ]) {
            update_entity_timeline(self, entity);
        }
    }
}
