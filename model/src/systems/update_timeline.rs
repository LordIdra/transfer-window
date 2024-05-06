use crate::{components::{vessel_component::timeline::TimelineEventType, ComponentType}, storage::entity_allocator::Entity, Model};

fn update_entity_timeline(model: &mut Model, entity: Entity) {
    let time = model.time();
    let events = model.vessel_component_mut(entity).timeline_mut().pop_events_before(time);
    for event in events {
        match event.type_() {
            TimelineEventType::FireTorpedo(fire_torpedo) => fire_torpedo.execute(model),
        }
    }
}

impl Model {
    pub fn update_timeline(&mut self) {
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            update_entity_timeline(self, entity);
        }
    }
}