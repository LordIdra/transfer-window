use nalgebra_glm::DVec2;

use crate::{components::vessel_component::{system_slot::SlotLocation, timeline::{fire_torpedo::FireTorpedoEvent, TimelineEvent, TimelineEventType}}, storage::entity_allocator::Entity, Model};

impl Model {
    #[allow(clippy::missing_panics_doc)]
    pub fn add_fire_torpedo_event(&mut self, fire_from: Entity, slot_location: SlotLocation, time: f64) {
        let event_type = TimelineEventType::FireTorpedo(FireTorpedoEvent::new(self, fire_from, time, slot_location));
        self.try_vessel_component_mut(fire_from)
            .unwrap()
            .timeline_mut()
            .add(TimelineEvent::new(time, event_type))  ;
    }

    /// # Panics
    /// Panics if there is no create torpedo event at the specified time
    pub fn adjust_fire_torpedo_event(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let fire_torpedo_event = self.fire_torpedo_event_at_time_mut(entity, time)
            .unwrap_or_else(|| panic!("Fire torpedo event not found at time {time}"));
        let entity = fire_torpedo_event.ghost();
        let time = fire_torpedo_event.burn_time();
        self.adjust_burn(entity, time, amount);
    }
}