use std::{collections::{HashMap, VecDeque}, fs};

use nalgebra_glm::vec2;
use serde::Deserialize;

use crate::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, state::State, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}};

use super::encounter::{Encounter, EncounterType};

#[derive(Deserialize)]
struct CaseMetaData {
    end_time: f64,
    time_step: f64,
}

#[derive(Debug, Deserialize)]
pub struct CaseEncounter {
    encounter_type: EncounterType,
    object: String,
    new_parent: String,
    time: f64,
}

impl CaseEncounter {
    pub fn get_type(&self) -> EncounterType {
        self.encounter_type.clone()
    }

    pub fn get_object(&self) -> String {
        self.object.clone()
    }

    pub fn get_new_parent(&self) -> String {
        self.new_parent.clone()
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn compare(&self, state: &State, encounter: &Encounter) -> bool {
        let object_name = state.get_name_component(encounter.get_entity()).get_name();
        let new_parent_name = state.get_name_component(encounter.get_new_parent()).get_name();
        let difference = (encounter.get_time() - self.get_time()).abs() / encounter.get_time();
        encounter.get_type() == self.get_type()
            && object_name == self.get_object()
            && new_parent_name == self.get_new_parent() 
            && difference < 0.005
    }
}

#[derive(Clone, Deserialize)]
struct CaseObjectData {
    orbitable: bool,
    mass: f64,
    position: [f64; 2],
    velocity: Option<[f64; 2]>,
    parent_name: Option<String>,
}

struct Case {
    metadata: CaseMetaData,
    object_data: Vec<CaseObjectData>,
    encounters: Vec<CaseEncounter>,
}

fn load_case_metadata(name: &str) -> CaseMetaData {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/metadata.json")
        .expect(format!("Failed to load metadata {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize metadata {}", name).as_str())
}

fn load_case_object_data(name: &str) -> HashMap<String, CaseObjectData> {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/objects.json")
        .expect(format!("Failed to load objects {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize objects {}", name).as_str())
}

fn load_case_encounters(name: &str) -> VecDeque<CaseEncounter> {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/encounters.json")
    .expect(format!("Failed to load encounters {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize objects {}", name).as_str())
}

pub fn load_case(name: &str) -> (State, VecDeque<CaseEncounter>, Entity, f64, f64) {
    let metadata = load_case_metadata(name);
    let mut object_data = load_case_object_data(name);
    let encounters = load_case_encounters(name);

    let mut state = State::mock();
    let mut object_entities: HashMap<String, Entity> = HashMap::new();
    let mut non_orbitable_entity = None;
    while !object_data.is_empty() {
        for (name, data) in object_data.clone() {
            let mut entity_builder = EntityBuilder::new()
                .with_name_component(NameComponent::new(name.clone()))
                .with_mass_component(MassComponent::new(data.mass));

            if data.velocity.is_none() && data.parent_name.is_none() {
                let position = vec2(data.position[0], data.position[1]);
                entity_builder = entity_builder.with_stationary_component(StationaryComponent::new(position));

            } else if data.velocity.is_some() && data.parent_name.is_some() {
                if let Some(parent) = object_entities.get(data.parent_name.as_ref().unwrap()) {
                    let parent_mass = state.get_mass_component(*parent).get_mass();
                    let position = vec2(data.position[0], data.position[1]);
                    let velocity = vec2(data.velocity.unwrap()[0], data.velocity.unwrap()[1]);
                    let mut trajectory_component = TrajectoryComponent::new();
                    trajectory_component.add_segment(Segment::Orbit(Orbit::new(*parent, data.mass, parent_mass, position, velocity, 0.0)));
                    entity_builder = entity_builder.with_trajectory_component(trajectory_component);
                } else {
                    continue; // the object's parent is not added yet
                }

            } else {
                panic!("Object {} has only one of velocity and parent_name, ", name);
            }

            if data.orbitable {
                entity_builder = entity_builder.with_orbitable_component(OrbitableComponent::new());
            }

            let entity = state.allocate(entity_builder);
            if !data.orbitable {
                match non_orbitable_entity {
                    Some(_) => panic!("Found multiple non-orbitable entities"),
                    None => non_orbitable_entity = Some(entity),
                }
            }

            object_entities.insert(name.clone(), entity);
            object_data.remove(&name);
        }
    }
    let non_orbitable_entity = non_orbitable_entity.expect("Case does not contain a non-orbitable entity");

    (state, encounters, non_orbitable_entity, metadata.end_time, metadata.time_step)
}