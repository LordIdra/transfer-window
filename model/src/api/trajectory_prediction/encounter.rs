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
    pub fn new(encounter_type: EncounterType, entity: Entity, new_parent: Entity, time: f64) -> Self {
        Self { encounter_type, entity, new_parent, time }
    }

    pub fn get_type(&self) -> EncounterType {
        self.encounter_type.clone()
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_new_parent(&self) -> Entity {
        self.new_parent
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    #[cfg(test)]
    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }
}
