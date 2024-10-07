use log::error;
use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::{path_component::orbit::Orbit, vessel_component::faction::Faction}, storage::entity_allocator::Entity};

use super::{snapshot::Snapshot, state_query::StateQuery, Model};

const DISTANCE_DERIVATIVE_DELTA: f64 = 0.1;

/// Step time needs to be large enough to be performant but small 
/// enough to catch all approaches. We choose this by choosing the 
/// minimum of the durations, and if applicable, periods, then dividing 
/// by a constant
fn compute_time_step(orbit_a: &Orbit, orbit_b: &Orbit, start_time: f64, end_time: f64) -> f64 {
    let mut step_time = end_time - start_time;
    if let Some(period) = orbit_a.period() {
        step_time = f64::min(step_time, period);
    }
    if let Some(period) = orbit_b.period() {
        step_time = f64::min(step_time, period);
    }
    step_time / 16.0
}

/// Returns an ordered vector of pairs of orbits that have the same parent
fn find_same_parent_orbit_pairs(snapshot: &Snapshot, entity_a: Entity, entity_b: Entity) -> Vec<(&Orbit, &Orbit)> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find same parent orbits");
    let orbits_a = snapshot.future_orbits(entity_a);
    let orbits_b = snapshot.future_orbits(entity_b);

    let mut same_parent_orbit_pairs = vec![];
    let mut index_a = 0;
    let mut index_b = 0;

    while index_a < orbits_a.len() && index_b < orbits_b.len() {
        let orbit_a = orbits_a[index_a];
        let orbit_b = orbits_b[index_b];

        if orbit_a.parent() == orbit_b.parent() {
            same_parent_orbit_pairs.push((orbit_a, orbit_b));
        }

        if orbit_a.end_point().time() < orbit_b.end_point().time() {
            index_a += 1;
        } else {
            index_b += 1;
        }
    }

    same_parent_orbit_pairs
}

/// Returns the time at which the next *perceived* closest approach will occur.
/// This ignore burns. Why? Picture two spacecraft getting closer
/// to each other. One of them starts burning to accelerate and
/// ends up moving away from the other spacecraft. This is logical
/// and makes sense, but in practice is very counterintuitive and
/// isn't really useful information eg when trying to plan a
/// rendezvous.
pub fn find_next_closest_approach(model: &Model, entity_a: Entity, entity_b: Entity, start_time: f64, observer: Option<Faction>) -> Option<f64> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find next closest approach");

    let snapshot = model.snapshot(start_time, observer);
    let same_parent_orbit_pairs = find_same_parent_orbit_pairs(&snapshot, entity_a, entity_b);

    for pair in same_parent_orbit_pairs {
        let orbit_a = pair.0;
        let orbit_b = pair.1;
        let start_time = f64::max(f64::max(orbit_b.start_point().time(), orbit_a.start_point().time()), start_time);
        let end_time = f64::min(orbit_a.end_point().time(), orbit_b.end_point().time());
        let time_step = compute_time_step(orbit_a, orbit_b, start_time, end_time);
        let distance = |time: f64| (orbit_a.point_at_time(time).position() - orbit_b.point_at_time(time).position()).magnitude();
        let distance_prime = |time: f64| (distance(time + DISTANCE_DERIVATIVE_DELTA) - distance(time)) / DISTANCE_DERIVATIVE_DELTA;

        if start_time > end_time {
            continue;
        }

        let mut time = start_time;
        let mut previous_time = time;
        let mut previous_distance_prime_value = distance_prime(time);
        time += time_step;

        loop {
            let distance_prime_value = distance_prime(time);
            if previous_distance_prime_value.is_sign_negative() && distance_prime_value.is_sign_positive() {
                let approach_time = itp(&distance_prime, previous_time, time);
                if let Err(err) = approach_time {
                    error!("Error while computing closest approach: {}", err);
                    return None;
                }
                return Some(approach_time.unwrap());
            }
            previous_time = time;
            previous_distance_prime_value = distance_prime_value;

            // This weird time step system is necessary because our time step might leave us in a position
            // where we overshoot the segment but don't actually evaluate the last point, so the entire 
            // final section of the segments is skipped
            if (time - end_time).abs() < 1.0e-3 {
                break
            }
            time += time_step;
            if time > end_time {
                time = end_time;
            }
        }
    }

    None
}

pub fn find_next_two_closest_approaches(
    model: &Model, 
    entity_a: Entity, 
    entity_b: Entity, 
    start_time: f64, 
    observer: Option<Faction>
) -> (Option<f64>, Option<f64>) {
    if let Some(time_1) = find_next_closest_approach(model, entity_a, entity_b, start_time, observer) {
        // Add 1.0 to make sure we don't find the same approach by accident
        if let Some(time_2) = find_next_closest_approach(model, entity_a, entity_b, time_1 + 1.0, observer) {
            return (Some(time_1), Some(time_2));
        }
        return (Some(time_1), None);
    }
    (None, None)
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::{orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{orbit::{builder::OrbitBuilder, orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}}, model::{closest_approach::{find_next_closest_approach, find_same_parent_orbit_pairs}, Model}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, test_util};

    #[test]
    fn test_find_same_parent_orbit_pairs() {
        let mut model = Model::default();

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None);
        let entity_builder = EntityBuilder::default();
        let entity_a = model.allocate(entity_builder.with_orbitable_component(orbitable));

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(1000.0, 0.0)), None);
        let entity_builder = EntityBuilder::default();
        let entity_b = model.allocate(entity_builder.with_orbitable_component(orbitable));

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(0.0, 1000.0)), None);
        let entity_builder = EntityBuilder::default();
        let entity_c = model.allocate(entity_builder.with_orbitable_component(orbitable));

        
        let mut path_component = PathComponent::default();

        let mut orbit_template = OrbitBuilder {
            parent: Entity::mock(),
            mass: 1.0e3,
            parent_mass: 1.0e23,
            rotation: 0.0,
            position: vec2(1.0e9, 0.0),
            velocity: vec2(0.0, 1.0e3),
            time: 0.0,
        };
    
        orbit_template.parent = entity_c;
        let segment_d_1 = Segment::Orbit(orbit_template.clone().build().with_end_at(10.0));
        path_component.add_segment(segment_d_1.clone());

        orbit_template.parent = entity_b;
        let segment_d_2 = Segment::Orbit(orbit_template.clone().build().with_end_at(50.0));
        path_component.add_segment(segment_d_2.clone());

        orbit_template.parent = entity_c;
        let segment_d_3 = Segment::Orbit(orbit_template.clone().build().with_end_at(100.0));
        path_component.add_segment(segment_d_3.clone());

        let entity_d = model.allocate(EntityBuilder::default().with_path_component(path_component));


        let mut path_component = PathComponent::default();

        orbit_template.parent = entity_a;
        let segment_e_1 = Segment::Orbit(orbit_template.clone().build().with_end_at(5.0));
        path_component.add_segment(segment_e_1.clone());

        orbit_template.parent = entity_b;
        let segment_e_2 = Segment::Orbit(orbit_template.clone().build().with_end_at(15.0));
        path_component.add_segment(segment_e_2.clone());

        orbit_template.parent = entity_c;
        let segment_e_3 = Segment::Orbit(orbit_template.clone().build().with_end_at(55.0));
        path_component.add_segment(segment_e_3.clone());

        orbit_template.parent = entity_a;
        let segment_e_4 = Segment::Orbit(orbit_template.clone().build().with_end_at(70.0));
        path_component.add_segment(segment_e_4.clone());

        orbit_template.parent = entity_c;
        let segment_e_5 = Segment::Orbit(orbit_template.clone().build().with_end_at(100.0));
        path_component.add_segment(segment_e_5.clone());

        let entity_e = model.allocate(EntityBuilder::default().with_path_component(path_component));


        let expected = vec![
            (segment_d_2, segment_e_2),
            (segment_d_3.clone(), segment_e_3),
            (segment_d_3, segment_e_5),
        ];

        let snapshot = model.snapshot_now();
        let actual = find_same_parent_orbit_pairs(&snapshot, entity_d, entity_e);

        assert_eq!(actual.len(), expected.len());

        for i in 0..actual.len() {
            assert!(expected[i].0.parent() == actual[i].0.parent());
            assert!(expected[i].1.parent() == actual[i].1.parent());
        }
    }

    #[test]
    fn test_find_next_closest_approach() {
        let mut model = Model::default();

        let sun = test_util::sun(&mut model);
        let earth = test_util::earth(&mut model, sun);

        let mut path_component = PathComponent::default();
        let orbit = Orbit::circle(earth, 3.0e2, 5.9722e24, vec2(0.1e9, 0.0), 0.0, OrbitDirection::Clockwise).with_end_at(1.0e10);
        path_component.add_segment(Segment::Orbit(orbit));
        let vessel_a = model.allocate(EntityBuilder::default().with_path_component(path_component));

        let mut path_component = PathComponent::default();
        let orbit = Orbit::circle(earth, 3.0e2, 5.9722e24, vec2(-0.1e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
        path_component.add_segment(Segment::Orbit(orbit.clone()));
        let vessel_b = model.allocate(EntityBuilder::default().with_path_component(path_component));

        let expected = orbit.period().unwrap() / 4.0;
        let actual = find_next_closest_approach(&model, vessel_a, vessel_b, 0.0, None).unwrap();

        println!("Actual: {actual} Expected: {expected}");
        assert!((expected - actual).abs() / expected < 1.0e-3);

        let expected = orbit.period().unwrap() * 3.0 / 4.0;
        let actual = find_next_closest_approach(&model, vessel_a, vessel_b, orbit.period().unwrap() / 2.0, None).unwrap();

        println!("Actual: {actual} Expected: {expected}");
        assert!((expected - actual).abs() / expected < 1.0e-3);
    }
}

