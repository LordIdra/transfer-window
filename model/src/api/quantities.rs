use log::error;
use nalgebra_glm::{vec2, DVec2};

use crate::{storage::entity_allocator::Entity, Model};

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
}