/// Incremental prediction is used at the start of the simulation and must only be run once
/// This needs to be done for all orbitable entities
/// Spacecraft do not need incremental prediction as they are not orbitable, so no other entities depend on them
/// Besides, we might need to recalculate spacecraft trajectories when eg a burn is adjusted
/// The way incremental prediction works is by predicting the trajectory of all orbitable entities at once
use std::collections::HashSet;

use crate::{storage::entity_allocator::Entity, state::State, components::{ComponentType, trajectory_component::{segment::Segment, orbit::Orbit}}, constants::{PREDICTION_END_TIME, PREDICTION_TIME_STEP}};

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

fn get_parallel_entities(state: &State, orbitable_entities_can_enter: &HashSet<Entity>, entity: Entity, parent: Entity) -> Vec<Entity> {
    let mut parallel_entities = vec![];
    for other_entity in orbitable_entities_can_enter {
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

/// Returns new parent if one was found
fn exit_check(state: &State, orbitable_entities_can_exit: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    if orbitable_entities_can_exit.contains(&entity) {
        let end_segment = state.get_trajectory_component(entity).get_end_segment();
        let position = end_segment.get_end_position();
        let parent = end_segment.get_parent();
        if position.magnitude() > sphere_of_influence(state, parent) {
            return Some(state.get_trajectory_component(parent).get_end_segment().get_parent());
        }
    }
    None
}

/// Returns new parent if one was found
fn entrance_check(state: &State, orbitable_entities_can_enter: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    let end_segment = state.get_trajectory_component(entity).get_end_segment();
    let position = end_segment.get_end_position();
    let parallel_entities = get_parallel_entities(state, orbitable_entities_can_enter, entity, end_segment.get_parent());
    for other_entity in parallel_entities {
        let other_position = state.get_trajectory_component(other_entity).get_end_segment().get_end_position();
        let distance = (position - other_position).magnitude();
        if distance < sphere_of_influence(state, other_entity) {
            return Some(other_entity);
        }
    }
    None
}

pub fn incremental_prediction(state: &mut State) {
    let mut time = 0.0;
    let orbiting_entities = state.get_entities(vec![ComponentType::TrajectoryComponent]);
    // entities whose SOI can be entered
    let orbitable_entities_can_enter = state.get_entities(vec![ComponentType::OrbitableComponent]);
    // entities whose SOI can be exited (ie not stationary objects)
    let orbitable_entities_can_exit = state.get_entities(vec![ComponentType::OrbitableComponent, ComponentType::TrajectoryComponent]);
    while time < PREDICTION_END_TIME {
        for entity in &orbiting_entities {
            state.get_trajectory_component_mut(*entity).get_end_segment_mut().as_orbit_mut().end_at(time);
            if let Some(new_parent) = exit_check(state, &orbitable_entities_can_exit, *entity) {
                let old_parent = state.get_trajectory_component(*entity).get_end_segment().get_parent();
                let new_parent_mass = state.get_mass_component(new_parent).get_mass();
                let position = state.get_trajectory_component(*entity).get_end_segment().get_end_position() + state.get_trajectory_component(old_parent).get_end_segment().get_end_position();
                let velocity = state.get_trajectory_component(*entity).get_end_segment().get_end_velocity() + state.get_trajectory_component(old_parent).get_end_segment().get_end_velocity();
                let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
                state.get_trajectory_component_mut(*entity).add_segment(segment);
            }
            if let Some(new_parent) = entrance_check(state, &orbitable_entities_can_enter, *entity) {
                let new_parent_mass = state.get_mass_component(new_parent).get_mass();
                let position = state.get_trajectory_component(*entity).get_end_segment().get_end_position() - state.get_trajectory_component(new_parent).get_end_segment().get_end_position();
                let velocity = state.get_trajectory_component(*entity).get_end_segment().get_end_velocity() - state.get_trajectory_component(new_parent).get_end_segment().get_end_velocity();
                let segment = Segment::Orbit(Orbit::new(new_parent, new_parent_mass, position, velocity, time));
                state.get_trajectory_component_mut(*entity).add_segment(segment);
            }
        }
        time += PREDICTION_TIME_STEP;
    }
}


#[cfg(test)]
mod test {
    fn test_incremental_prediction() {

    }
}