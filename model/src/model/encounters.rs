use crate::{components::vessel_component::faction::Faction, storage::entity_allocator::Entity};

use super::{state_query::StateQuery, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncounterType {
    Entrance,
    Exit,
}

pub struct Encounter {
    time: f64,
    encounter_type: EncounterType,
    from: Entity,
    to: Entity,
}

impl Encounter {
    pub fn new(time: f64, encounter_type: EncounterType, from: Entity, to: Entity) -> Self {
        Self { time, encounter_type, from, to }
    }
    
    pub fn time(&self) -> f64 {
        self.time
    }
    
    pub fn encounter_type(&self) -> EncounterType {
        self.encounter_type
    }
    
    pub fn from(&self) -> Entity {
        self.from
    }
    
    pub fn to(&self) -> Entity {
        self.to
    }
}

pub fn future_encounters(model: &Model, entity: Entity, time: f64, observer: Option<Faction>) -> Vec<Encounter> {
        let mut encounters = vec![];
        let mut previous_parent = None;
        for orbit in model.snapshot(time, observer).future_orbits(entity) {
            let Some(previous_parent) = previous_parent else {
                previous_parent = Some(orbit.parent());
                continue;
            };

            let new_parent = orbit.parent();
            if new_parent == previous_parent {
                continue;
            }

            let encounter_type = match model.orbitable_component(previous_parent).segment() {
                Some(previous_parent_orbit) => {
                    if previous_parent_orbit.parent() == new_parent {
                        EncounterType::Exit
                    } else {
                        EncounterType::Entrance
                    }
                }
                None => EncounterType::Entrance,
            };

            let time = orbit.start_point().time();
            encounters.push(Encounter::new(time, encounter_type, previous_parent, new_parent));
        }

        encounters
    }
