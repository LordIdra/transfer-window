use crate::{components::{path_component::guidance::MAX_GUIDANCE_TIME, vessel_component::{system_slot::{Slot, SlotLocation}, timeline::fire_torpedo::FireTorpedoEvent}}, storage::entity_allocator::Entity, Model};

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

    pub fn can_edit(&self, entity: Entity) -> bool {
        self.vessel_component(entity).can_edit_ever() 
            && self.vessel_component(entity).timeline().events().is_empty() 
            && self.path_component(entity).final_burn().is_none()
    }

    pub fn can_torpedo_enable_guidance(&self, entity: Entity, time: f64) -> bool {
        let vessel_component = &self.vessel_component(entity);
        let Some(target) = vessel_component.target() else {
            return false;
        };

        if !vessel_component.class().is_torpedo() {
            return false;
        };

        let Some(next_closest_approach_time) = self.find_next_closest_approach(entity, target, time) else {
            return false;
        };

        if next_closest_approach_time - time >= MAX_GUIDANCE_TIME {
            return false;
        }

        true

        // strategy: while guidance enabled:
        // if intersection distance > optimal distance
        // compute derivative at intersection time
        // 
    }
}