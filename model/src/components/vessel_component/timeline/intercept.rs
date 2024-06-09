use serde::{Deserialize, Serialize};

use crate::{api::explosion::Explosion, storage::entity_allocator::Entity, Model};

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

    #[allow(clippy::missing_panics_doc)]
    pub fn execute(&self, model: &mut Model) {
        let parent = model.parent_at_time(self.entity, self.time).unwrap();
        let offset = model.position_at_time(self.entity, self.time);
        let combined_mass = model.mass(self.entity) + model.mass(self.target);
        model.add_explosion(Explosion::new(parent, offset, combined_mass));
        model.deallocate(self.entity);
        model.deallocate(self.target);
    }

    pub fn cancel(&self, _model: &mut Model) {}

    pub fn is_blocking(&self) -> bool {
        false
    }

    pub fn can_remove(&self) -> bool {
        false
    }

    pub fn can_adjust(&self) -> bool {
        false
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn traget(&self) -> Entity {
        self.target
    }

    pub fn time(&self) -> f64 {
        self.time
    }

}