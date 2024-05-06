use crate::{components::vessel_component::timeline::TimelineEvent, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn cancel_last_event(&mut self, fire_from: Entity) {
        self.vessel_component_mut(fire_from).timeline_mut().pop_last_event().cancel(self);
    }

    pub fn add_event(&mut self, entity: Entity, event: TimelineEvent) {
        self.vessel_component_mut(entity)
            .timeline_mut()
            .add(event);
    }

    /// # Panics
    /// Panics if there is no create torpedo event at the specified time
    pub fn event_at_time(&self, entity: Entity, time: f64) -> &TimelineEvent {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)
            .unwrap()
    }

    pub fn can_modify_timeline_event(&self, entity: Entity, time: f64) -> bool {
        match self.vessel_component(entity).timeline().last_event() {
            Some(event) => event.time() == time,
            None => false,
        }
    }
}