use crate::{components::path_component::{orbit::{builder::OrbitBuilder, Orbit}, segment::Segment}, model::{state_query::StateQuery, Model}, storage::entity_allocator::Entity};

use super::encounter::{Encounter, EncounterType};

mod bounding;
pub mod solver;

pub fn calculate_exit_encounter(model: &Model, orbit: &Orbit, new_parent: Entity, time: f64) -> Orbit {
    let old_parent = orbit.parent();
    let old_parent_point = &model.orbitable_component(old_parent).orbit().unwrap().point_at_time(time);
    
    OrbitBuilder {
        parent: new_parent,
        mass: orbit.mass(),
        parent_mass: model.mass(new_parent),
        rotation: orbit.rotation(),
        position: orbit.end_point().position() + old_parent_point.position(),
        velocity: orbit.end_point().velocity() + old_parent_point.velocity(),
        time,
    }.build()
}

pub fn calculate_entrance_encounter(model: &Model, orbit: &Orbit, new_parent: Entity, time: f64) -> Orbit {
    let new_parent_point = &model.orbitable_component(new_parent).orbit().unwrap().point_at_time(time);

    OrbitBuilder {
        parent: new_parent,
        mass: orbit.mass(),
        parent_mass: model.mass(new_parent),
        rotation: orbit.rotation(),
        position: orbit.end_point().position() - new_parent_point.position(),
        velocity: orbit.end_point().velocity() - new_parent_point.velocity(),
        time,
    }.build()
}

fn do_exit(model: &mut Model, entity: Entity, new_parent: Entity, time: f64) {
    model.path_component_mut(entity)
        .end_segment_mut()
        .as_orbit_mut()
        .expect("Attempt to do an exit encounter on a non-orbit")
        .end_at(time);
    let old_orbit = model.path_component(entity)
        .end_segment()
        .as_orbit()
        .expect("Attempt to do an exit encounter on a non-orbit");

    let new_orbit = calculate_exit_encounter(model, old_orbit, new_parent, time);

    model.path_component_mut(entity).add_segment(Segment::Orbit(new_orbit));
}

fn do_entrance(model: &mut Model, entity: Entity, new_parent: Entity, time: f64) {
    model.path_component_mut(entity)
        .end_segment_mut()
        .as_orbit_mut()
        .expect("Attempt to do an entrance encounter on a non-orbit")
        .end_at(time);
    let old_orbit = model.path_component(entity)
        .end_segment()
        .as_orbit()
        .expect("Attempt to do an entrance encounter on a non-orbit");

    let new_orbit = calculate_entrance_encounter(model, old_orbit, new_parent, time);

    model.path_component_mut(entity).add_segment(Segment::Orbit(new_orbit));
}

/// This detachment of encounter solving and application 
/// allows the solver to be much more easily tested, as well 
/// as leading to cleaner overall design
pub fn apply_encounter(model: &mut Model, encounter: &Encounter) {
    match encounter.type_() {
        EncounterType::Entrance => do_entrance(model, encounter.entity(), encounter.new_parent(), encounter.time()),
        EncounterType::Exit => do_exit(model, encounter.entity(), encounter.new_parent(), encounter.time()),
    }
}

#[cfg(test)]
mod test {
    use crate::{components::ComponentType, model::trajectories::{encounter::Encounter, fast_solver::{apply_encounter, solver::find_next_encounter}, test_cases::load_case}};

    fn run_case(name: &str) {
        let (mut model, mut encounters, _, end_time, _) = load_case(name);

        loop {
            let mut soonest_encounter: Option<Encounter> = None;
            for entity in model.entities(vec![ComponentType::PathComponent]) {
                let orbit = model.path_component(entity).end_orbit().unwrap();
                let encounter = find_next_encounter(&model, orbit, entity, end_time).unwrap();
                if let Some(encounter) = encounter {
                    if let Some(soonest_encounter) = &mut soonest_encounter {
                        if encounter.time() < soonest_encounter.time() {
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
            encounter.set_time(case_encounter.time());

            for entity in model.entities(vec![ComponentType::PathComponent]) {
                model.path_component_mut(entity)
                    .end_segment_mut()
                    .as_orbit_mut()
                    .expect("Attempt to set end time of a segment that is not an orbit")
                    .end_at(encounter.time());
            }

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
