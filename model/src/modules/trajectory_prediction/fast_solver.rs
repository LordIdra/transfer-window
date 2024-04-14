use crate::{components::trajectory_component::{orbit::Orbit, segment::Segment}, Model, storage::entity_allocator::Entity};

use super::encounter::{Encounter, EncounterType};

mod bounding;
pub mod solver;

fn do_exit(model: &mut Model, entity: Entity, new_parent: Entity, time: f64) {
    let old_parent = model.get_trajectory_component(entity).get_end_segment().get_parent();
    let mass = model.get_mass_component(entity).get_mass();
    let new_parent_mass = model.get_mass_component(new_parent).get_mass();
    let position = model.get_trajectory_component(entity).get_end_segment().get_end_position() + model.get_trajectory_component(old_parent).get_end_segment().get_end_position();
    let velocity = model.get_trajectory_component(entity).get_end_segment().get_end_velocity() + model.get_trajectory_component(old_parent).get_end_segment().get_end_velocity();
    model.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(time);
    let segment = Segment::Orbit(Orbit::new(new_parent, mass, new_parent_mass, position, velocity, time));
    model.get_trajectory_component_mut(entity).add_segment(segment);
}

fn do_entrance(model: &mut Model, entity: Entity, new_parent: Entity, time: f64) {
    let new_parent_mass = model.get_mass_component(new_parent).get_mass();
    let mass = model.get_mass_component(entity).get_mass();
    let position = model.get_trajectory_component(entity).get_end_segment().get_end_position() - model.get_trajectory_component(new_parent).get_end_segment().get_end_position();
    let velocity = model.get_trajectory_component(entity).get_end_segment().get_end_velocity() - model.get_trajectory_component(new_parent).get_end_segment().get_end_velocity();
    model.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(time);
    let segment = Segment::Orbit(Orbit::new(new_parent, mass, new_parent_mass, position, velocity, time));
    model.get_trajectory_component_mut(entity).add_segment(segment);
}

/// This detachment of encounter solving and application allows the solver to be much more easily tested
/// As well as leading to cleaner overall design
pub fn apply_encounter(model: &mut Model, encounter: &Encounter) {
    match encounter.get_type() {
        EncounterType::Entrance => do_entrance(model, encounter.get_entity(), encounter.get_new_parent(), encounter.get_time()),
        EncounterType::Exit => do_exit(model, encounter.get_entity(), encounter.get_new_parent(), encounter.get_time()),
    }
}

#[cfg(test)]
mod test {
    use crate::{components::ComponentType, modules::trajectory_prediction::{fast_solver::{apply_encounter, solver::find_next_encounter}, test_cases::load_case, encounter::Encounter}};

    fn run_case(name: &str) {
        let (mut model, mut encounters, _, end_time, _) = load_case(name);

        let mut start_time = 0.0;
        loop {
            let mut soonest_encounter: Option<Encounter> = None;
            for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
                let encounter = find_next_encounter(&model, entity, start_time, end_time);
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
                panic!("Found unexpected encounter: {encounter:#?}");
            };
            assert!(next_encounter.compare(&model, &encounter), "Encounters not equal: {:#?} {:#?}", encounter, encounters.front());

            // We unfortunately have to use the case encounter's time, not the calculated time
            // Small errors in the cases or differences in implementations can cause the simulations to massively diverge in more complex tests
            let case_encounter = encounters.pop_front().unwrap();
            encounter.set_time(case_encounter.get_time());

            for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
                model.get_trajectory_component_mut(entity).get_end_segment_mut().as_orbit_mut().end_at(encounter.get_time());
            }

            start_time = encounter.get_time();
            apply_encounter(&mut model, &encounter);
        }

        assert!(encounters.is_empty(), "Missed encounters: {encounters:#?}");
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