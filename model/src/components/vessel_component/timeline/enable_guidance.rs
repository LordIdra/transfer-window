use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

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
        // TODO
        // model.delete_guidance(self.entity, self.time);
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}