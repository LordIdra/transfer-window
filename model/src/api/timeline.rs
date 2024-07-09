use log::debug;

use crate::{components::vessel_component::timeline::{fire_torpedo::FireTorpedoEvent, TimelineEvent}, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn cancel_last_event(&mut self, entity: Entity) {
        let event = &self.vessel_component_mut(entity).timeline_mut().pop_last_event();
        debug!("Cancelled timeline event {event:?}");
        event.cancel(self);
    }

    pub fn add_event(&mut self, entity: Entity, event: TimelineEvent) {
        debug!("Adding event {event:?}");
        self.vessel_component_mut(entity)
            .timeline_mut()
            .add(event);
    }

    /// # Panics
    /// Panics if there is no create torpedo event at the specified time
    pub fn timeline_event_at_time(&self, entity: Entity, time: f64) -> &TimelineEvent {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)
            .unwrap()
    }

    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.vessel_component(entity).timeline().event_at_time(time)?.as_fire_torpedo()
    }
}