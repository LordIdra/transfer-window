use serde::Deserialize;

use crate::{components::trajectory_component::{orbit::Orbit, segment::Segment}, state::State, storage::entity_allocator::Entity};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub enum EncounterType {
    Entrance,
    Exit,
}

#[derive(Debug)]
pub struct Encounter {
    encounter_type: EncounterType,
    object: Entity,
    new_parent: Entity,
    time: f64,
}

impl Encounter {
    pub fn new(encounter_type: EncounterType, object: Entity, new_parent: Entity, time: f64) -> Self {
        Self { encounter_type, object, new_parent, time }
    }

    pub fn get_type(&self) -> EncounterType {
        self.encounter_type.clone()
    }

    pub fn get_object(&self) -> Entity {
        self.object
    }

    pub fn get_new_parent(&self) -> Entity {
        self.new_parent
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}

fn do_exit(state: &mut State, entity: Entity, new_parent: Entity, time: f64) {
    let old_parent = state.get_trajectory_component(entity).get_end_segment().get_parent();
    let new_parent_mass = state.get_mass_component(new_parent).get_mass();
    let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() + state.get_trajectory_component(old_parent).get_end_segment().get_end_position();
    let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() + state.get_trajectory_component(old_parent).get_end_segment().get_end_velocity();
    let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
    state.get_trajectory_component_mut(entity).add_segment(segment);
}

fn do_entrance(state: &mut State, entity: Entity, new_parent: Entity, time: f64) {
    let new_parent_mass = state.get_mass_component(new_parent).get_mass();
    let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() - state.get_trajectory_component(new_parent).get_end_segment().get_end_position();
    let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() - state.get_trajectory_component(new_parent).get_end_segment().get_end_velocity();
    let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
    state.get_trajectory_component_mut(entity).add_segment(segment);
}

/// This detachment of encounter solving and application allows the solver to be much more easily tested
/// As well as leading to cleaner overall design
pub fn apply_encounter(state: &mut State, encounter: Encounter) {
    match encounter.encounter_type {
        EncounterType::Entrance => do_entrance(state, encounter.object, encounter.new_parent, encounter.time),
        EncounterType::Exit => do_exit(state, encounter.object, encounter.new_parent, encounter.time),
    }
}