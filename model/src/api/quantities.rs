use log::error;
use nalgebra_glm::{vec2, DVec2};

use crate::{components::trajectory_component::segment::Segment, storage::entity_allocator::Entity, Model};

impl Model {
    pub fn get_position(&self, entity: Entity) -> Option<DVec2> {
        if let Some(stationary_component) = self.try_get_stationary_component(entity) {
            return Some(stationary_component.get_position())
        }

        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            return Some(trajectory_component.get_current_segment().get_current_position())
        }

        None
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_absolute_position(&self, entity: Entity) -> DVec2 {
        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            let current_segment = trajectory_component.get_current_segment();
            return self.get_absolute_position(current_segment.get_parent()) + current_segment.get_current_position();
        }

        if let Some(stationary_component) = self.try_get_stationary_component(entity) {
            return stationary_component.get_position();
        }

        error!("Request to get absolute position of entity without trajectory or stationary components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_absolute_velocity(&self, entity: Entity) -> DVec2 {
        if let Some(trajectory_component) = self.try_get_trajectory_component(entity) {
            let current_segment = trajectory_component.get_current_segment();
            return self.get_absolute_velocity(current_segment.get_parent()) + current_segment.get_current_velocity();
        }

        if self.try_get_stationary_component(entity).is_some() {
            return vec2(0.0, 0.0);
        }

        error!("Request to get absolute position of entity without trajectory or stationary components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    pub fn get_mass(&self, entity: Entity) -> f64 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_mass();
        }

        if let Some(vessel_component) = self.try_get_vessel_component(entity) {
            if let Segment::Burn(burn) = self.get_trajectory_component(entity).get_current_segment() {
                return burn.get_current_point().get_mass();
            }
            return vessel_component.get_mass();
        }

        error!("Request to get mass of entity without orbitable or vessel components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }

    pub fn get_mass_at_time(&self, entity: Entity, time: f64) -> f64 {
        if let Some(orbitable_component) = self.try_get_orbitable_component(entity) {
            return orbitable_component.get_mass()
        }

        if let Some(vessel_component) = self.try_get_vessel_component(entity) {

            // find last burn before time if it exists
            for segment in self.get_trajectory_component(entity).get_segments().iter().flatten().rev() {
                if segment.get_start_time() > time {
                    continue;
                }

                if let Segment::Burn(burn) = segment {
                    if time > segment.get_start_time() && time < segment.get_end_time() {
                        // The requested time is within the burn
                        return burn.get_point_at_time(time).get_mass();
                    }
                    
                    // Otherwise, the requested time is after the burn, so return mass at end of burn
                    return burn.get_end_point().get_mass();
                }
            }
            return vessel_component.get_mass()
        }

        error!("Request to get mass of entity without orbitable or vessel components");
        panic!("Error recoverable, but exiting anyway before something bad happens");
    }
}