use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

const MIN_DV_TO_ENABLE_GUIDANCE: f64 = 1.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnableGuidanceEvent {
    entity: Entity,
    time: f64,
}

impl EnableGuidanceEvent {
    pub fn new(model: &mut Model, entity: Entity, time: f64) -> Self {
        model.create_guidance(entity, time);
        Self { entity, time }
    }

    pub fn execute(&self, _model: &mut Model) {}

    pub fn cancel(&self, model: &mut Model) {
        model.delete_guidance(self.entity, self.time);
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
        model.vessel_component(entity).class().is_torpedo()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn can_create(model: &Model, entity: Entity, time: f64) -> bool {
        let vessel_component = model.vessel_component(entity);
        vessel_component.timeline().is_time_after_last_blocking_event(time)
            && !vessel_component.timeline().last_event().is_some_and(|event| event.is_intercept())
            && vessel_component.target().is_some_and(|target| model.try_vessel_component(target).is_some())
            && model.end_dv(entity).unwrap() > MIN_DV_TO_ENABLE_GUIDANCE
    }
}
