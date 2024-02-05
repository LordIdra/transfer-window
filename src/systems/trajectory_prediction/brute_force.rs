/// Brute force incremental prediction does the following:
/// - steps the time
/// - updates each conic to end at that time
/// - checks for SOI changes at that time
/// - if SOI change found, creates a new conic at that time
/// This method is extremely reliable but very slow, so we can use it to test the reliability of faster methods
/// Also, it's comparatively inaccurate (only as accurate as the time step) and doesn't make any attempt to refine past what if finds

use std::collections::HashSet;

use crate::{components::{trajectory_component::{orbit::Orbit, segment::Segment}, ComponentType}, state::State, storage::entity_allocator::Entity};

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

fn do_exit(state: &mut State, can_exit: &HashSet<Entity>, entity: Entity, time: f64) {
    if let Some(new_parent) = exit_check(state, &can_exit, entity) {
        let old_parent = state.get_trajectory_component(entity).get_end_segment().get_parent();
        let new_parent_mass = state.get_mass_component(new_parent).get_mass();
        let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() + state.get_trajectory_component(old_parent).get_end_segment().get_end_position();
        let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() + state.get_trajectory_component(old_parent).get_end_segment().get_end_velocity();
        let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
        state.get_trajectory_component_mut(entity).add_segment(segment);
    }
}

fn do_entrance(state: &mut State, can_enter: &HashSet<Entity>, entity: Entity, time: f64) {
    if let Some(new_parent) = entrance_check(state, &can_enter, entity) {
        let new_parent_mass = state.get_mass_component(new_parent).get_mass();
        let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() - state.get_trajectory_component(new_parent).get_end_segment().get_end_position();
        let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() - state.get_trajectory_component(new_parent).get_end_segment().get_end_velocity();
        let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
        state.get_trajectory_component_mut(entity).add_segment(segment);
    }
}

/// Incremental prediction is used at the start of the simulation and must only be run once
/// This needs to be done for all orbitable entities
/// Spacecraft do not need incremental prediction as they are not orbitable, so no other entities depend on them
/// Besides, we might need to recalculate spacecraft trajectories when eg a burn is adjusted
/// The way incremental prediction works is by predicting the trajectory of all orbitable entities at once
pub fn incremental_prediction(state: &mut State, end_time: f64, time_step: f64) {
    let mut time = 0.0;
    let entities_to_predict = state.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]);
    while time < end_time {
        for entity in &entities_to_predict {
            state.get_trajectory_component_mut(*entity).get_end_segment_mut().as_orbit_mut().end_at(time);
            do_exit(state, &entities_to_predict, entity.clone(), time);
            do_entrance(state, &entities_to_predict, entity.clone(), time);
        }
        time += time_step;
    }
}

/// Used to calculate the trajectory of one object (ie a spacecraft)
/// Expects that all orbitables have had their trajectory until end_time computed
pub fn oneshot_prediction(state: &mut State, entity: Entity, start_time: f64, end_time: f64, time_step: f64) {
    let mut time = start_time;
    let can_enter_or_exit = state.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]);
    while time < end_time {
        state.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(time);
        do_exit(state, &can_enter_or_exit, entity, time);
        do_entrance(state, &can_enter_or_exit, entity, time);
        time += time_step;
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;
    use serde::Deserialize;

    use crate::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, state::State, storage::entity_builder::EntityBuilder, systems::trajectory_prediction::brute_force::oneshot_prediction};

    use super::incremental_prediction;

    #[test]
    fn test_incremental_simple() {
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

        incremental_prediction(&mut state, 13.997 * 24.0 * 60.0 * 60.0, 60.0);

        let moon_position = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point().get_position();
        let moon_velocity = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point().get_velocity();
        assert!((moon_position.magnitude() - 4.156e8).abs() < 1.0e5);
        assert!((moon_velocity.magnitude() - 946.0).abs() < 1.0);

        incremental_prediction(&mut state, 365.169 * 24.0 * 60.0 * 60.0, 60.0);

        let earth_position = state.get_trajectory_component(earth).get_end_segment().as_orbit().get_end_point().get_position();
        let earth_velocity = state.get_trajectory_component(earth).get_end_segment().as_orbit().get_end_point().get_velocity();
        assert!((earth_position.magnitude() - 147.095e9).abs() < 1.0e5);
        assert!((earth_velocity.magnitude() - 30.29e3).abs() < 1.0);
    }

    #[test]
    fn test_incremental_prediction_with_encounter() {
        let mut state = State::mock();

        let sun = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Sun".to_string()))
            .with_mass_component(MassComponent::new(1.989e30))
            .with_orbitable_component(OrbitableComponent::new())
            .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

        let mut earth_trajectory = TrajectoryComponent::new();
        earth_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 1.989e30, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 0.0)));
        let earth = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Earth".to_string()))
            .with_mass_component(MassComponent::new(5.972e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(earth_trajectory));

        let mut moon_trajectory = TrajectoryComponent::new();
        moon_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 1.989e30, vec2(1.525e11, -0.05e11), vec2(0.0, -2.7e4), 0.0)));
        let moon = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Moon".to_string()))
            .with_mass_component(MassComponent::new(0.07346e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(moon_trajectory));

        assert!(state.get_trajectory_component(moon).get_end_segment().get_parent() == sun);
        incremental_prediction(&mut state, 21.0 * 24.0 * 60.0 * 60.0, 60.0);
        assert!(state.get_trajectory_component(moon).get_end_segment().get_parent() == earth);
        incremental_prediction(&mut state, 28.0 * 24.0 * 60.0 * 60.0, 60.0);
        assert!(state.get_trajectory_component(moon).get_end_segment().get_parent() == sun);
    }

    #[test]
    fn test_oneshot_simple() {
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
        incremental_prediction(&mut state, end_time, 60.0);
        oneshot_prediction(&mut state, moon, 0.0, end_time, 60.0);

        let moon_position = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point().get_position();
        let moon_velocity = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point().get_velocity();
        assert!((moon_position.magnitude() - 4.156e8).abs() < 1.0e5);
        assert!((moon_velocity.magnitude() - 946.0).abs() < 1.0);
    }

    fn test_external_cases() {
        
    }
}
