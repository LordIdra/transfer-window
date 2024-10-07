use serde::{Deserialize, Serialize};

use crate::{model::{explosion::Explosion, state_query::StateQuery, Model}, storage::entity_allocator::Entity};

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
        let snapshot = model.snapshot_at(self.time);
        let parent = snapshot.parent(self.entity).unwrap();
        let offset = snapshot.position(self.entity);
        let combined_mass = snapshot.mass(self.entity) + snapshot.mass(self.target);
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
