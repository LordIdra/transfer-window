use crate::{components::vessel_component::{ship::ship_slot::{ShipSlot, ShipSlotLocation}, timeline::fire_torpedo::FireTorpedoEvent}, storage::entity_allocator::Entity, Model};

impl Model {
    /// # Panics
    /// 
    pub fn set_slot(&mut self, entity: Entity, location: ShipSlotLocation, slot: ShipSlot) {
        self.vessel_component_mut(entity).as_ship_mut().unwrap().set_slot(location, slot);
        self.recompute_entire_trajectory(entity);
    }

    pub fn fire_torpedo_event_at_time(&self, entity: Entity, time: f64) -> Option<FireTorpedoEvent> {
        self.vessel_component(entity).timeline().event_at_time(time)?.as_fire_torpedo()
    }

    pub fn target(&self, entity: Entity) -> Option<Entity> {
        self.try_vessel_component(entity)?.target()
    }

    pub fn can_edit(&self, entity: Entity) -> bool {
        self.vessel_component(entity).can_edit_ever() 
            && self.vessel_component(entity).timeline().events().is_empty() 
            && self.path_component(entity).final_burn().is_none()
    }
}