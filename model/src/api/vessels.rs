use crate::{components::{path_component::segment::Segment, vessel_component::{system_slot::{Slot, SlotLocation}, timeline::fire_torpedo::FireTorpedoEvent}}, storage::entity_allocator::Entity, Model};

const MAX_GUIDANCE_ORBIT_FRACTION: f64 = 0.2;
const MAX_GUIDANCE_ANGLE: f64 = 0.2;

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

    pub fn can_torpedo_enable_guidance(&self, entity: Entity) -> bool {
        let vessel_component = &self.vessel_component(entity);
        let Some(target) = vessel_component.target() else {
            return false;
        };

        if !vessel_component.class().is_torpedo() {
            return false;
        };

        let Some(next_closest_approach) = self.find_next_closest_approach(entity, target, self.time) else {
            return false;
        };

        let current_segment = self.path_component(entity).current_segment();
        let segment_at_approach = self.path_component(entity).future_segment_at_time(next_closest_approach);
        if current_segment.start_time() != segment_at_approach.start_time() {
            return false;
        }

        let Segment::Orbit(orbit) = current_segment else {
            return false;
        };

        let time_to_encounter = next_closest_approach - self.time;
        if let Some(period) = orbit.period() {
            if time_to_encounter / period > MAX_GUIDANCE_ORBIT_FRACTION {
                return false;
            }
        }

        if self.velocity(entity).angle(&self.velocity_at_time(entity, next_closest_approach)) > MAX_GUIDANCE_ANGLE {
            return false;
        }

        return true;

        // strategy: while guidance enabled:
        // if intersection distance > optimal distance
        // compute derivative at intersection time
        // 
    }
}