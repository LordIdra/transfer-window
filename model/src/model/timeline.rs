use log::debug;

use crate::{components::vessel_component::timeline::{fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent, start_turn::StartTurnEvent, TimelineEvent}, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn cancel_last_event(&mut self, entity: Entity) {
        let event = &self.vessel_component_mut(entity)
            .timeline_mut()
            .pop_last_event();
        debug!("Cancelled timeline event {event:?}");
        event.cancel(self);
    }

    pub fn add_event(&mut self, entity: Entity, event: TimelineEvent) {
        debug!("Adding event {event:?}");
        self.vessel_component_mut(entity)
            .timeline_mut()
            .add(event);
    }

    pub fn can_delete_event_at_time(&self, entity: Entity, time: f64) -> bool {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)
            .unwrap()
            .can_delete(self)
    }

    pub fn can_adjust_event_at_time(&self, entity: Entity, time: f64) -> bool {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)
            .unwrap()
            .can_adjust(self)
    }

    pub fn start_burn_event_at_time(&self, entity: Entity, time: f64) -> Option<StartBurnEvent> {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)?
            .as_start_burn()
    }

    pub fn start_turn_event_at_time(&self, entity: Entity, time: f64) -> Option<StartTurnEvent> {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)?
            .as_start_turn()
    }

    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)?
            .as_fire_torpedo()
    }
}
