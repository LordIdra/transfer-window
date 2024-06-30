use crate::{components::vessel_component::timeline::fire_torpedo::FireTorpedoEvent, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.vessel_component(entity).timeline().event_at_time(time)?.as_fire_torpedo()
    }

    pub fn target(&self, entity: Entity) -> Option<Entity> {
        self.try_vessel_component(entity)?.target()
    }
}