/// Brute force incremental prediction does the following:
/// - steps the time
/// - updates each conic to end at that time
/// - checks for SOI changes at that time
/// - if SOI change found, creates a new conic at that time
/// This method is extremely reliable but very slow, so we can use it to test the reliability of faster methods
/// Also, it's comparatively inaccurate (only as accurate as the time step) and doesn't make any attempt to refine past what if finds

use std::collections::HashSet;

use crate::{components::ComponentType, state::State, storage::entity_allocator::Entity};

use super::util::{get_final_siblings, Encounter, EncounterType};

fn exit_check(state: &State, can_exit: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    let trajectory_component = state.get_trajectory_component(entity);
    let end_segment = trajectory_component.get_end_segment();
    let position = end_segment.get_end_position();
    let parent = end_segment.get_parent();
    if can_exit.contains(&parent) {
        let sphere_of_influence = state.get_trajectory_component(parent).get_end_segment().as_orbit().get_sphere_of_influence();
        let distance = position.magnitude();
        if can_exit.contains(&parent) && distance > sphere_of_influence {
            return Some(state.get_trajectory_component(parent).get_end_segment().get_parent());
        }
    }
    None
}

fn entrance_check(state: &State, can_enter: &HashSet<Entity>, entity: Entity) -> Option<Entity> {
    let end_segment = state.get_trajectory_component(entity).get_end_segment();
    let position = end_segment.get_end_position();
    let siblings = get_final_siblings(state, can_enter, entity);
    for sibling in siblings {
        let trajectory_component = state.get_trajectory_component(sibling);
        let end_segment = trajectory_component.get_end_segment();
        let sphere_of_influence = end_segment.as_orbit().get_sphere_of_influence();
        let other_position = end_segment.get_end_position();
        let distance = (position - other_position).magnitude();
        if distance < sphere_of_influence {
            return Some(sibling);
        }
    }
    None
}

pub fn incremental_find_next_encounter(state: &mut State, start_time: f64, end_time: f64, time_step: f64) -> Option<Encounter> {
    let mut time = start_time;
    let entities = state.get_entities(vec![ComponentType::TrajectoryComponent]);
    while time < end_time {
        for entity in &entities {
            if let Some(new_parent) = entrance_check(state, &entities, *entity) {
                return Some(Encounter::new(EncounterType::Entrance, *entity, new_parent, time));
            }
            if let Some(new_parent) = exit_check(state, &entities, *entity) {
                return Some(Encounter::new(EncounterType::Exit, *entity, new_parent, time));
            }
        }
        time += time_step;
        for entity in &entities {
            state.get_trajectory_component_mut(*entity).get_end_segment_mut().as_orbit_mut().end_at(time);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, state::State, storage::entity_builder::EntityBuilder, systems::trajectory_prediction::{test_cases::load_case, util::apply_encounter}};

    use super::incremental_find_next_encounter;

    fn run_case(name: &str) {
        let (mut state, mut encounters, _, end_time, time_step) = load_case(name);

        let mut start_time = 0.0;
        while let Some(encounter) = incremental_find_next_encounter(&mut state, start_time, end_time, time_step) {
            let Some(next_encounter) = encounters.front() else {
                panic!("Found unexpected encounter: {:#?}", encounter);
            };
            if !next_encounter.compare(&state, &encounter) {
                panic!("Encounters not equal: {:#?} {:#?}", encounter, encounters.front())
            }
            encounters.pop_front();
            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        if !encounters.is_empty() {
            panic!("Missed encounters: {:?}", encounters);
        }
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
        earth_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 5.972e24, 1.989e30, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0)));
        let earth = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Earth".to_string()))
            .with_mass_component(MassComponent::new(5.972e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(earth_trajectory));

        let mut moon_trajectory = TrajectoryComponent::new();
        moon_trajectory.add_segment(Segment::Orbit(Orbit::new(earth, 0.07346e24, 5.972e24, vec2(0.3633e9, 0.0), vec2(0.0, 1.082e3), 0.0)));
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
        earth_trajectory.add_segment(Segment::Orbit(Orbit::new(sun, 5.972e24, 1.989e30, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0)));
        let earth = state.allocate(EntityBuilder::new()
            .with_name_component(NameComponent::new("Earth".to_string()))
            .with_mass_component(MassComponent::new(5.972e24))
            .with_orbitable_component(OrbitableComponent::new())
            .with_trajectory_component(earth_trajectory));

        let mut moon_trajectory = TrajectoryComponent::new();
        moon_trajectory.add_segment(Segment::Orbit(Orbit::new(earth, 0.07346e24, 5.972e24, vec2(0.3633e9, 0.0), vec2(0.0, 1.082e3), 0.0)));
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

        let end_point = state.get_trajectory_component(moon).get_end_segment().as_orbit().get_end_point();
        assert!((end_point.get_position().magnitude() - 4.156e8).abs() < 1.0e5);
        assert!((end_point.get_velocity().magnitude() - 946.0).abs() < 1.0);
        assert!((end_point.get_time() - end_time).abs() < 61.0);
    }

    #[test]
    fn test_case_collision_with_moon() {
        run_case("collision-with-moon");
    }

    #[test]
    fn test_case_ellipse_encounter_with_escaping_moon() {
        run_case("ellipse-encounter-with-escaping-moon");
    }

    #[test]
    fn test_case_encounter_with_earth() {
        run_case("encounter-with-earth");
    }
    
    #[test]
    fn test_case_encounter_with_earth_and_moon() {
        run_case("encounter-with-earth-and-moon");
    }

    #[test]
    fn test_case_escape_from_earth() {
        run_case("escape-from-earth");
    }

    #[test]
    fn test_case_hyperbola_encounter_with_escaping_moon() {
        run_case("hyperbola-encounter-with-escaping-moon");
    }

    #[test]
    fn test_case_insanity_1() {
        run_case("insanity-1");
    }

    #[test]
    fn test_case_insanity_2() {
        run_case("insanity-2");
    }

    #[test]
    fn test_case_insanity_3() {
        run_case("insanity-3");
    }

    #[test]
    fn test_case_many_moon_encounters() {
        run_case("many-moon-encounters");
    }

    #[test]
    fn test_case_moon_slingshot_to_escape_earth() {
        run_case("moon-slingshot-to-escape-earth");
    }

    #[test]
    fn test_case_no_encounters() {
        run_case("no-encounters");
    }

    #[test]
    fn test_case_two_moons_varied_encounter() {
        run_case("two-moons-varied-encounters");
    }
}