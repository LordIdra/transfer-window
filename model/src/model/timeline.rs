use log::debug;

use crate::{components::vessel_component::timeline::{start_guidance::StartGuidanceEvent, fire_torpedo::FireTorpedoEvent, intercept::InterceptEvent, start_burn::StartBurnEvent, start_turn::StartTurnEvent, TimelineEvent}, storage::entity_allocator::Entity};

use super::Model;

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

    pub fn event(&self, entity: Entity, time: f64) -> Option<&TimelineEvent> {
        self.vessel_component(entity)
            .timeline()
            .event_at_time(time)
    }

    pub fn intercept_event_at_time(&self, entity: Entity, time: f64) -> Option<InterceptEvent> {
        self.event(entity, time)?.as_intercept()
    }

    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.event(entity, time)?.as_fire_torpedo()
    }

    pub fn start_burn_event_at_time(&self, entity: Entity, time: f64) -> Option<StartBurnEvent> {
        self.event(entity, time)?.as_start_burn()
    }

    pub fn start_turn_event_at_time(&self, entity: Entity, time: f64) -> Option<StartTurnEvent> {
        self.event(entity, time)?.as_start_turn()
    }

    pub fn start_guidance_event_at_time(&self, entity: Entity, time: f64) -> Option<StartGuidanceEvent> {
        self.event(entity, time)?.as_start_guidance()
    }
}
