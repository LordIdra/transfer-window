use crate::{components::trajectory_component::{orbit::Orbit, segment::Segment}, state::State, storage::entity_allocator::Entity};

use super::encounter::{Encounter, EncounterType};

mod bounding;
mod solver;

fn do_exit(state: &mut State, entity: Entity, new_parent: Entity, time: f64) {
    let old_parent = state.get_trajectory_component(entity).get_end_segment().get_parent();
    let mass = state.get_mass_component(entity).get_mass();
    let new_parent_mass = state.get_mass_component(new_parent).get_mass();
    let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() + state.get_trajectory_component(old_parent).get_end_segment().get_end_position();
    let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() + state.get_trajectory_component(old_parent).get_end_segment().get_end_velocity();
    let segment = Segment::Orbit(Orbit::new(new_parent, mass, new_parent_mass, position, velocity, time));
    state.get_trajectory_component_mut(entity).add_segment(segment);
}

fn do_entrance(state: &mut State, entity: Entity, new_parent: Entity, time: f64) {
    let new_parent_mass = state.get_mass_component(new_parent).get_mass();
    let mass = state.get_mass_component(entity).get_mass();
    let position = state.get_trajectory_component(entity).get_end_segment().get_end_position() - state.get_trajectory_component(new_parent).get_end_segment().get_end_position();
    let velocity = state.get_trajectory_component(entity).get_end_segment().get_end_velocity() - state.get_trajectory_component(new_parent).get_end_segment().get_end_velocity();
    let segment = Segment::Orbit(Orbit::new(new_parent, mass, new_parent_mass, position, velocity, time));
    state.get_trajectory_component_mut(entity).add_segment(segment);
}

/// This detachment of encounter solving and application allows the solver to be much more easily tested
/// As well as leading to cleaner overall design
pub fn apply_encounter(state: &mut State, encounter: Encounter) {
    match encounter.get_type() {
        EncounterType::Entrance => do_entrance(state, encounter.get_entity(), encounter.get_new_parent(), encounter.get_time()),
        EncounterType::Exit => do_exit(state, encounter.get_entity(), encounter.get_new_parent(), encounter.get_time()),
    }
}

// #[cfg(test)]
pub mod test {
    use crate::{components::ComponentType, systems::trajectory_prediction::{fast_solver::{apply_encounter, solver::find_next_encounter}, test_cases::load_case, encounter::Encounter}};

    // pub fn run_cases() {
    //     run_case("collision-with-moon");
    //     run_case("encounter-with-earth");
    //     run_case("encounter-with-earth-and-moon");
    //     run_case("escape-from-earth");
    //     run_case("escape-from-moon-1");
    //     run_case("escape-from-moon-2");
    //     run_case("hyperbolic-moon-encounter-1");
    //     run_case("hyperbolic-moon-encounter-2");
    //     run_case("hyperbolic-moon-encounter-3");
    //     run_case("hyperbolic-moon-encounter-4");
    //     run_case("hyperbolic-moon-encounter-5");
    //     run_case("insanity-1");
    //     run_case("insanity-2");
    //     run_case("insanity-3");
    //     run_case("many-moon-encounters");
    //     run_case("moon-slingshot-to-escape-earth");
    //     run_case("no-encounters");
    //     run_case("parallel-with-moon");
    //     run_case("two-moons-varied-encounters-1");
    //     run_case("two-moons-varied-encounters-2");
    //     run_case("two-moons-varied-encounters-3");
    // }

    fn run_case(name: &str) {
        let (mut state, mut encounters, _, end_time, _) = load_case(name);

        let mut start_time = 0.0;
        loop {
            let mut soonest_encounter: Option<Encounter> = None;
            for entity in state.get_entities(vec![ComponentType::TrajectoryComponent]) {
                let encounter = find_next_encounter(&state, entity, start_time, end_time);
                if let Some(encounter) = encounter {
                    if let Some(soonest_encounter) = &mut soonest_encounter {
                        if encounter.get_time() < soonest_encounter.get_time() {
                            *soonest_encounter = encounter;
                        }
                    } else {
                        soonest_encounter = Some(encounter);
                    }
                }
            }

            if soonest_encounter.is_none() {
                break;
            }
            
            let mut encounter = soonest_encounter.unwrap();
            
            let Some(next_encounter) = encounters.front() else {
                panic!("Found unexpected encounter: {:#?}", encounter);
            };
            if !next_encounter.compare(&state, &encounter) {
                panic!("Encounters not equal: {:#?} {:#?}", encounter, encounters.front())
            }

            // We unfortunately have to use the case encounter's time, not the calculated time
            // Small errors in the cases or differences in implementations can cause the simulations to massively diverge in more complex tests
            let case_encounter = encounters.pop_front().unwrap();
            encounter.set_time(case_encounter.get_time());

            for entity in state.get_entities(vec![ComponentType::TrajectoryComponent]) {
                state.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(encounter.get_time());
            }

            start_time = encounter.get_time();
            apply_encounter(&mut state, encounter);
        }

        if !encounters.is_empty() {
            panic!("Missed encounters: {:#?}", encounters);
        }
    }

    #[test]
    pub fn test_case_collision_with_moon() {
        run_case("collision-with-moon");
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
    fn test_case_escape_from_moon_1() {
        run_case("escape-from-moon-1");
    }

    #[test]
    fn test_case_escape_from_moon_2() {
        run_case("escape-from-moon-2");
    }

    #[test]
    fn test_case_hyperbolic_moon_encounter_1() {
        run_case("hyperbolic-moon-encounter-1");
    }

    #[test]
    fn test_case_hyperbolic_moon_encounter_2() {
        run_case("hyperbolic-moon-encounter-2");
    }

    #[test]
    fn test_case_hyperbolic_moon_encounter_3() {
        run_case("hyperbolic-moon-encounter-3");
    }

    #[test]
    fn test_case_hyperbolic_moon_encounter_4() {
        run_case("hyperbolic-moon-encounter-4");
    }

    #[test]
    fn test_case_hyperbolic_moon_encounter_5() {
        run_case("hyperbolic-moon-encounter-5");
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
    fn test_case_parallel_with_moon() {
        run_case("parallel-with-moon");
    }

    #[test]
    fn test_case_two_moons_varied_encounter_1() {
        run_case("two-moons-varied-encounters-1");
    }

    #[test]
    fn test_case_two_moons_varied_encounter_2() {
        run_case("two-moons-varied-encounters-2");
    }

    #[test]
    fn test_case_two_moons_varied_encounter_3() {
        run_case("two-moons-varied-encounters-3");
    }
}