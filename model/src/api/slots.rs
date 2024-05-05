use crate::{components::vessel_component::system_slot::{Slot, SlotLocation}, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn set_slot(&mut self, entity: Entity, location: SlotLocation, slot: Slot) {
        self.vessel_component_mut(entity).set_slot(location, slot);
        self.recompute_entire_trajectory(entity);
    }
}