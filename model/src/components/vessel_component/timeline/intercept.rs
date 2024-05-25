use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterceptEvent {
    entity: Entity,
    target: Entity,
    time: f64,
}

impl InterceptEvent {
    pub fn new(_model: &mut Model, entity: Entity, target: Entity, time: f64) -> Self {
        Self { entity, target, time }
    }

    pub fn execute(&self, model: &mut Model) {
        model.deallocate(self.entity);
        model.deallocate(self.target);
    }

    pub fn cancel(&self, _model: &mut Model) {}

    pub fn time(&self) -> f64 {
        self.time
    }
}