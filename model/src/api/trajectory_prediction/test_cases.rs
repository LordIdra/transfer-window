use std::{collections::{HashMap, VecDeque}, fs};

use nalgebra_glm::vec2;
use serde::Deserialize;

use crate::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{orbit::Orbit, segment::Segment, PathComponent}, vessel_component::{VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

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

    pub fn compare(&self, model: &Model, encounter: &Encounter) -> bool {
        let object_name = model.get_name_component(encounter.get_entity()).get_name();
        let new_parent_name = model.get_name_component(encounter.get_new_parent()).get_name();
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

fn load_case_metadata(name: &str) -> CaseMetaData {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/metadata.json")
        .unwrap_or_else(|_| panic!("Failed to load metadata {name}"));
    serde_json::from_str(file.as_str())
        .unwrap_or_else(|_| panic!("Failed to deserialize metadata {name}"))
}

fn load_case_object_data(name: &str) -> HashMap<String, CaseObjectData> {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/objects.json")
        .unwrap_or_else(|_| panic!("Failed to load objects {name}"));
    serde_json::from_str(file.as_str())
        .unwrap_or_else(|_| panic!("Failed to deserialize objects {name}"))
}

fn load_case_encounters(name: &str) -> VecDeque<CaseEncounter> {
    let file = fs::read_to_string("resources/prediction-test-cases/".to_string() + name + "/encounters.json")
    .unwrap_or_else(|_| panic!("Failed to load encounters {name}"));
    serde_json::from_str(file.as_str())
        .unwrap_or_else(|_| panic!("Failed to deserialize objects {name}"))
}

pub fn load_case(name: &str) -> (Model, VecDeque<CaseEncounter>, Entity, f64, f64) {
    let metadata = load_case_metadata(name);
    let mut object_data = load_case_object_data(name);
    let encounters = load_case_encounters(name);

    let mut model = Model::default();
    let mut object_entities: HashMap<String, Entity> = HashMap::new();
    let mut non_orbitable_entity = None;
    while !object_data.is_empty() {
        for (name, data) in object_data.clone() {
            let mut entity_builder = EntityBuilder::default()
                .with_name_component(NameComponent::new(name.clone()));

            let position = vec2(data.position[0], data.position[1]);
            if data.parent_name.is_some() {
                let Some(parent) = object_entities.get(data.parent_name.as_ref().unwrap()) else {
                    // parent not added yet
                    continue;
                };

                let Some(velocity) = data.velocity else {
                    panic!("An object has parent but not velocity");
                };

                let parent_mass = model.get_mass(*parent);
                let velocity = vec2(velocity[0], velocity[1]);
                let orbit = Orbit::new(*parent, data.mass, parent_mass, position, velocity, 0.0);

                if data.orbitable {
                    let orbitable_component = OrbitableComponent::new(data.mass, 0.0, OrbitableComponentPhysics::Stationary(position));
                    entity_builder = entity_builder.with_orbitable_component(orbitable_component);
                } else {
                    let path_component = PathComponent::default()
                        .with_segment(Segment::Orbit(orbit));
                    entity_builder = entity_builder.with_path_component(path_component);
                    entity_builder = entity_builder.with_vessel_component(VesselComponent::new(VesselClass::Light));
                }
            } else {
                entity_builder = entity_builder.with_orbitable_component(OrbitableComponent::new(data.mass, 0.0, OrbitableComponentPhysics::Stationary(position)));
            }

            let entity = model.allocate(entity_builder);
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

    (model, encounters, non_orbitable_entity, metadata.end_time, metadata.time_step)
}