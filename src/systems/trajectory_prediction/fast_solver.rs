mod bounding;
mod entity_solver;

#[cfg(test)]
mod test {
    use crate::{components::ComponentType, systems::trajectory_prediction::{fast_solver::entity_solver::find_next_encounter, test_cases::load_case, util::{apply_encounter, Encounter}}};

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
            
            println!("{:?}", encounter);

            let Some(next_encounter) = encounters.front() else {
                panic!("Found unexpected encounter: {:#?}", encounter);
            };
            if !next_encounter.compare(&state, &encounter) {
                panic!("Encounters not equal: {:#?} {:#?}", encounter, encounters.front())
            }

            // We unfortunately have to use the case encounter's time, not the calculated time
            // Small errors in the cases can cause the simulations to massively diverge in more complex tests
            //TODO stop doing this if we can get more accurate cases
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