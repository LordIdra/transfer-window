use std::{collections::HashMap, fs};

use nalgebra_glm::vec2;
use serde::Deserialize;

use crate::{components::{mass_component::MassComponent, name_component::NameComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, state::State, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}};

#[derive(Deserialize)]
struct MetaData {
    end_time: f64,
    time_step: f64,
}

#[derive(Deserialize)]
struct ObjectData {
    orbitable: bool,
    mass: f64,
    position: [f64; 2],
    velocity: Option<[f64; 2]>,
    parent_name: Option<String>,
}

#[derive(Deserialize)]
enum EncounterType {
    Entrance,
    Exit,
}

#[derive(Deserialize)]
struct Encounter {
    encounter_type: EncounterType,
    object: String,
    new_parent: String,
    time: f64,
}

struct Case {
    metadata: MetaData,
    object_data: Vec<ObjectData>,
    encounters: Vec<Encounter>,
}

fn load_case_metadata(name: &String) -> MetaData {
    let file = fs::read_to_string("cases/".to_string() + name.as_str() + "/metadata.json")
        .expect(format!("Failed to load metadata {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize metadata {}", name).as_str())
}

fn load_case_object_data(name: &String, metadata: &MetaData) -> HashMap<String, ObjectData> {
    let file = fs::read_to_string("cases/".to_string() + name.as_str() + "/objects.json")
        .expect(format!("Failed to load objects {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize objects {}", name).as_str())
}

fn load_case_encounters(name: &String) -> Vec<Encounter> {
    let file = fs::read_to_string("cases/".to_string() + name.as_str() + "/encounters.json")
    .expect(format!("Failed to load encounters {}", name).as_str());
    serde_json::from_str(file.as_str())
        .expect(format!("Failed to deserialize objects {}", name).as_str())
}

fn load_case(name: &String) -> (State, Vec<Encounter>, f64) {
    let metadata = load_case_metadata(name);
    let object_data = load_case_object_data(name, &metadata);
    let encounters = load_case_encounters(name);

    let mut state = State::mock();
    let changed = true;
    let mut object_entities: HashMap<String, Entity> = HashMap::new();
    while changed {
        for (name, data) in &object_data {
            let mut entity_builder = EntityBuilder::new()
                .with_name_component(NameComponent::new(name.clone()))
                .with_mass_component(MassComponent::new(data.mass));
            if data.velocity.is_none() && data.parent_name.is_none() {
                entity_builder = entity_builder.with_stationary_component(StationaryComponent::new(vec2(data.position[0], data.position[1])));
                object_entities.insert(name.clone(), state.allocate(entity_builder));
            } else if data.velocity.is_some() && data.parent_name.is_some() {
                if object_entities.get(data.parent_name.unwrap()) {
                    let trajectory_component = TrajectoryComponent::new();
                    trajectory_component.add_segment(Segment::Orbit(Orbit::new(parent, parent_mass, position, velocity, time)))
                    entity_builder = entity_builder.with_trajectory_component();
                }
            } else {
                panic!("Object {} has only one of velocity and parent_name, ", name);
            }
        }
    }

    (state, metadata.end_time)
}