use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BurnEvent {
    entity: Entity,
    time: f64,
}

impl BurnEvent {
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
}