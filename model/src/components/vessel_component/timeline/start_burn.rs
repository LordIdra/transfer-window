use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartBurnEvent {
    entity: Entity,
    time: f64,
}

impl StartBurnEvent {
    pub fn new(model: &mut Model, entity: Entity, time: f64) -> Self {
        let rocket_equation_function = model.rocket_equation_function_at_end_of_trajectory(entity);
        model.create_burn(entity, time, rocket_equation_function);
        Self { entity, time }
    }

    pub fn execute(&self, _model: &mut Model) {}

    pub fn cancel(&self, model: &mut Model) {
        model.delete_burn(self.entity, self.time);
    }

    pub fn adjust(&self, model: &mut Model, amount: DVec2) {
        model.adjust_burn(self.entity, self.time, amount);
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn is_blocking(&self) -> bool {
        true
    }

    pub fn can_remove(&self, model: &Model) -> bool {
        model.vessel_component(self.entity).timeline().is_time_after_last_blocking_event(self.time)
    }

    pub fn can_adjust(&self, model: &Model) -> bool {
        model.vessel_component(self.entity).timeline().is_time_after_last_blocking_event(self.time)
    }

    pub fn can_create_ever() -> bool {
        true
    }

    pub fn can_create(model: &Model, entity: Entity, time: f64) -> bool {
        model.vessel_component(entity).timeline().is_time_after_last_blocking_event(time)
            && model.vessel_component(entity).slots().engine().is_some()
            && !model.vessel_component(entity).slots().fuel_tanks().is_empty()
    }
}