/// Brute force incremental prediction does the following:
/// - steps the time
/// - updates each conic to end at that time
/// - checks for SOI changes at that time
/// - if SOI change found, creates a new conic at that time
/// This method is extremely reliable but very slow, so we can use it to test the reliability of faster methods
/// Also, it's comparatively inaccurate (only as accurate as the time step) and doesn't make any attempt to refine past what if finds

use std::collections::HashSet;

use crate::{components::ComponentType, state::State, storage::entity_allocator::Entity};

use super::util::{Encounter, EncounterType};

/// https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
fn sphere_of_influence(state: &State, entity: Entity) -> f64 {
    let trajectory_component = state.get_trajectory_component(entity);
    let end_segment = trajectory_component.get_end_segment();
    let parent = end_segment.get_parent();
    let semi_major_axis = end_segment.as_orbit().get_semi_major_axis();
    let mass = state.get_mass_component(entity).get_mass();
    let parent_mass = state.get_mass_component(parent).get_mass();
    semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
}

fn get_parallel_entities(state: &State, can_enter: &HashSet<Entity>, entity: Entity, parent: Entity) -> Vec<Entity> {
    let mut parallel_entities = vec![];
    for other_entity in can_enter {
        if entity == *other_entity {
            continue;
        }
        let other_end_segment = state.get_trajectory_component(*other_entity).get_end_segment();
        if parent != other_end_segment.get_parent() {
            continue;
        }
        parallel_entities.push(*other_entity);
    }
    parallel_entities
}

 fn exit_check(state: &State, can_exit: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    if can_exit.contains(&entity) {
        let end_segment = state.get_trajectory_component(entity).get_end_segment();
        let position = end_segment.get_end_position();
        let parent = end_segment.get_parent();
        if can_exit.contains(&parent) && position.magnitude() > sphere_of_influence(state, parent) {
            return Some(state.get_trajectory_component(parent).get_end_segment().get_parent());
        }
    }
    None
}

fn entrance_check(state: &State, can_enter: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    let end_segment = state.get_trajectory_component(entity).get_end_segment();
    let position = end_segment.get_end_position();
    let parallel_entities = get_parallel_entities(state, can_enter, entity, end_segment.get_parent());
    for other_entity in parallel_entities {
        let other_position = state.get_trajectory_component(other_entity).get_end_segment().get_end_position();
        let distance = (position - other_position).magnitude();
        if distance < sphere_of_influence(state, other_entity) {
            return Some(other_entity);
        }
    }
    None
}

/// Incremental prediction is used at the start of the simulation and must only be run once
/// This needs to be done for all orbitable entities
/// Spacecraft do not need incremental prediction as they are not orbitable, so no other entities depend on them
/// Besides, we might need to recalculate spacecraft trajectories when eg a burn is adjusted
/// The way incremental prediction works is by predicting the trajectory of all orbitable entities at once
pub fn incremental_find_next_encounter(state: &mut State, start_time: f64, end_time: f64, time_step: f64) -> Option<Encounter> {
    let mut time = start_time;
    let entities = state.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]);
    while time < end_time {
        for entity in &entities {
            state.get_trajectory_component_mut(*entity).get_end_segment_mut().as_orbit_mut().end_at(time);
            if let Some(new_parent) = exit_check(state, &entities, *entity) {
                return Some(Encounter::new(EncounterType::Exit, *entity, new_parent, time));
            }
            if let Some(new_parent) = entrance_check(state, &entities, *entity) {
                return Some(Encounter::new(EncounterType::Entrance, *entity, new_parent, time));
            }
        }
        time += time_step;
    }
    None
}

/// Used to calculate the trajectory of one object (ie a spacecraft)
/// Expects that all orbitables have had their trajectory until end_time computed
pub fn oneshot_find_next_encounter(state: &mut State, entity: Entity, start_time: f64, end_time: f64, time_step: f64) -> Option<Encounter> {
    let mut time = start_time;
    let entities = state.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]);
    while time < end_time {
        state.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(time);
        if let Some(new_parent) = exit_check(state, &entities, entity) {
            return Some(Encounter::new(EncounterType::Exit, entity, new_parent, time));
        }
        if let Some(new_parent) = entrance_check(state, &entities, entity) {
            return Some(Encounter::new(EncounterType::Entrance, entity, new_parent, time));
        }
        time += time_step;
    }
    None
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, state::State, storage::entity_builder::EntityBuilder, systems::trajectory_prediction::{brute_force::oneshot_find_next_encounter, test_cases::{load_case, CaseEncounter}, util::{apply_encounter, Encounter}}};

    use super::incremental_find_next_encounter;

    fn compare_encounters(state: &State, encounter: &Encounter, case_encounter: &CaseEncounter) -> bool {
        let object_name = state.get_name_component(encounter.get_object()).get_name();
        let new_parent_name = state.get_name_component(encounter.get_new_parent()).get_name();
        encounter.get_type() == case_encounter.get_type()
            && object_name == case_encounter.get_object()
            && new_parent_name == case_encounter.get_new_parent()
            && (encounter.get_time() - case_encounter.get_time()).abs() < 100.0
    }

    #[test]
    fn test_incremental() {
        let mut state = State::mock();
        
        let sun = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Sun".to_string()))
            .with_mass_component(MassComponent::new(1.989e30))
            .with_orbitable_component(OrbitableComponent::new())
            .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

        let mut earth_trajectory = TrajectoryComponent::new();
        earth_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 1.989e30, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0)));
        let earth = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Earth".to_string()))
            .with_mass_component(MassComponent::new(5.972e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(earth_trajectory));

        let mut moon_trajectory = TrajectoryComponent::new();
        moon_trajectory.add_segment(Segment::Orbit(Orbit::new(earth, 5.972e24, vec2(0.3633e9, 0.0), vec2(0.0, 1.082e3), 0.0)));
        let moon = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Moon".to_string()))
            .with_mass_component(MassComponent::new(0.07346e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(moon_trajectory));

        let mut start_time = 0.0;
        let end_time = 13.997 * 24.0 * 60.0 * 60.0;
        while let Some(encounter) = incremental_find_next_encounter(&mut state, start_time, end_time, 60.0) {
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        let end_point = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point();
        assert!((end_point.get_position().magnitude() - 4.156e8).abs() < 1.0e5);
        assert!((end_point.get_velocity().magnitude() - 946.0).abs() < 1.0);
        assert!((end_point.get_time() - end_time).abs() < 61.0);

        let mut start_time = 0.0;
        let end_time = 365.169 * 24.0 * 60.0 * 60.0;
        while let Some(encounter) = incremental_find_next_encounter(&mut state, start_time, end_time, 60.0) {
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        let end_point = state.get_trajectory_component(earth).get_end_segment().as_orbit().get_end_point();
        assert!((end_point.get_position().magnitude() - 147.095e9).abs() < 1.0e5);
        assert!((end_point.get_velocity().magnitude() - 30.29e3).abs() < 1.0);
        assert!((end_point.get_time() - end_time).abs() < 61.0);
    }

    #[test]
    fn test_oneshot() {
        let mut state = State::mock();

        let sun = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Sun".to_string()))
            .with_mass_component(MassComponent::new(1.989e30))
            .with_orbitable_component(OrbitableComponent::new())
            .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

        let mut earth_trajectory = TrajectoryComponent::new();
        earth_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 1.989e30, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0)));
        let earth = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Earth".to_string()))
            .with_mass_component(MassComponent::new(5.972e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(earth_trajectory));

        let mut moon_trajectory = TrajectoryComponent::new();
        moon_trajectory.add_segment(Segment::Orbit(Orbit::new(earth, 5.972e24, vec2(0.3633e9, 0.0), vec2(0.0, 1.082e3), 0.0)));
        let moon = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Moon".to_string()))
            .with_mass_component(MassComponent::new(0.07346e24))
            .with_trajectory_component(moon_trajectory));

        let end_time = 13.997 * 24.0 * 60.0 * 60.0;
        let mut start_time = 0.0;
        while let Some(encounter) = incremental_find_next_encounter(&mut state, start_time, end_time, 60.0) {
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }
        let mut start_time = 0.0;
        while let Some(encounter) = oneshot_find_next_encounter(&mut state, moon, start_time, end_time, 60.0) {
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        let end_point = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point();
        assert!((end_point.get_position().magnitude() - 4.156e8).abs() < 1.0e5);
        assert!((end_point.get_velocity().magnitude() - 946.0).abs() < 1.0);
        assert!((end_point.get_time() - end_time).abs() < 61.0);
    }

    // TODO - document the fuck out of this, it's kinda confusing
    #[test]
    fn test_case_no_encounters() {
        let (mut state, mut encounters, non_orbitable_entity, end_time, time_step) = load_case("escape-from-earth");

        let mut start_time = 0.0;
        while let Some(encounter) = incremental_find_next_encounter(&mut state, start_time, end_time, time_step) {
            println!("1 {:?}", encounter);
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        let mut start_time = 0.0;
        while let Some(encounter) = oneshot_find_next_encounter(&mut state, non_orbitable_entity, start_time, end_time, time_step) {
            println!("2 {:?}", encounter);
            if !compare_encounters(&state, &encounter, encounters.front().expect("Found unexpected encounter")) {
                panic!("Encounters not equal: {:?} {:?}", encounter, encounters.front())
            }
            encounters.pop_front();
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }
        println!("{:#?}", state);
        if !encounters.is_empty() {
            panic!("Missed encounters: {:?}", encounters);
        }
    }
}

// find, tar, gzip, nohup, paralle, basename, gnuplot