use crate::{storage::entity_allocator::Entity, Model};

#[derive(Debug, Clone, Copy)]
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
    fn new(time: f64, encounter_type: EncounterType, from: Entity, to: Entity) -> Self {
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

impl Model {
    pub fn future_encounters(&self, entity: Entity) -> Vec<Encounter> {
        let mut encounters = vec![];
        let mut previous_parent = None;
        for orbit in self.path_component(entity).future_orbits() {
            if let Some(previous_parent) = previous_parent {
                let new_parent = orbit.parent();
                if new_parent != previous_parent {
                    let encounter_type = match self.orbitable_component(previous_parent).orbit() {
                        Some(previous_parent_orbit) => {
                            match previous_parent_orbit.parent() == new_parent {
                                true => EncounterType::Exit,
                                false => EncounterType::Entrance,
                            }
                        }
                        None => EncounterType::Entrance,
                    };
                    let time = orbit.start_point().time();
                    encounters.push(Encounter::new(time, encounter_type, previous_parent, new_parent));
                }
            }
            previous_parent = Some(orbit.parent());
        }
        encounters
    }
}