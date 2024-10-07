use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

use crate::{model::{state_query::StateQuery, Model}, storage::entity_allocator::Entity};

const MIN_DV_TO_CREATE_BURN: f64 = 1.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartBurnEvent {
    entity: Entity,
    time: f64,
}

impl StartBurnEvent {
    pub fn new(model: &mut Model, entity: Entity, time: f64) -> Self {
        model.create_burn(entity, time, vec2(0.01, 0.0));
        Self { entity, time }
    }

    pub fn execute(&self, _model: &mut Model) {}

    pub fn cancel(&self, model: &mut Model) {
        model.delete_segment(self.entity, self.time);
    }

    pub fn adjust(&self, model: &mut Model, amount: DVec2) {
        let dv = model.snapshot_at(self.burn_segment_time(model)).burn_starting_now(self.entity).delta_v();
        model.create_burn(self.entity, self.time, dv + amount);
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn burn_segment_time(&self, model: &Model) -> f64 {
        model.snapshot_at(self.time).turn_starting_now(self.entity).end_point().time()
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
            && model.end_dv(entity).unwrap() > MIN_DV_TO_CREATE_BURN
    }
}
