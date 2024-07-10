use serde::Deserialize;

use crate::storage::entity_allocator::Entity;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub enum EncounterType {
    Entrance,
    Exit,
}

#[derive(Debug)]
pub struct Encounter {
    encounter_type: EncounterType,
    entity: Entity,
    new_parent: Entity,
    time: f64,
}

impl Encounter {
    pub fn new(
        encounter_type: EncounterType,
        entity: Entity,
        new_parent: Entity,
        time: f64,
    ) -> Self {
        Self {
            encounter_type,
            entity,
            new_parent,
            time,
        }
    }

    pub fn type_(&self) -> EncounterType {
        self.encounter_type.clone()
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn new_parent(&self) -> Entity {
        self.new_parent
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    #[cfg(test)]
    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }
}
