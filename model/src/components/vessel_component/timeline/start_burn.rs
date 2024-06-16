use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

const MIN_DV_TO_CREATE_BURN: f64 = 1.0;

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

    pub fn can_create_ever(model: &Model, entity: Entity) -> bool {
        model.vessel_component(entity).has_engine()
            && model.vessel_component(entity).has_fuel_tank()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn can_create(model: &Model, entity: Entity, time: f64) -> bool {
        let vessel_component = model.vessel_component(entity);
        vessel_component.timeline().is_time_after_last_blocking_event(time)
            && !vessel_component.timeline().last_event().is_some_and(|event| event.is_enable_guidance() || event.is_intercept())
            && model.final_dv(entity).unwrap() > MIN_DV_TO_CREATE_BURN
    }
}