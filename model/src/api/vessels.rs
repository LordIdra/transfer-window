use crate::{components::vessel_component::{system_slot::{Slot, SlotLocation}, timeline::fire_torpedo::FireTorpedoEvent}, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn set_slot(&mut self, entity: Entity, location: SlotLocation, slot: Slot) {
        self.vessel_component_mut(entity).set_slot(location, slot);
        self.recompute_entire_trajectory(entity);
    }

    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.vessel_component(entity).timeline().event_at_time(time)?.type_().as_fire_torpedo()
    }

    pub fn target(&self, entity: Entity) -> Option<Entity> {
        self.try_vessel_component(entity)?.target()
    }

    pub fn can_change_target(&self, entity: Entity) -> bool {
        match self.try_vessel_component(entity) {
            Some(vessel_component) => vessel_component.can_change_target(),
            None => false,
        }
    }

    pub fn can_edit(&self, entity: Entity) -> bool {
        self.vessel_component(entity).can_edit_ever() 
            && self.vessel_component(entity).timeline().events().is_empty() 
            && self.path_component(entity).final_burn().is_none()
    }
}