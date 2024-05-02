use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::trajectory_component::segment::Segment, storage::entity_allocator::Entity, Model};

/// Returns an ordered vector of pairs of orbits that have the same parent
fn find_same_parent_orbit_pairs(model: &Model, entity_a: Entity, entity_b: Entity) -> Vec<(&Segment, &Segment)> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Find same parent orbits");
    let mut same_parent_orbit_pairs = vec![];
    let segments_a: Vec<&Segment> = model.get_trajectory_component(entity_a).get_segments().iter().flatten().collect();
    let segments_b: Vec<&Segment> = model.get_trajectory_component(entity_b).get_segments().iter().flatten().collect();
    let mut index_a = 0;
    let mut index_b = 0;

    while index_a < segments_a.len() && index_b < segments_b.len() {
        let segment_a = segments_a[index_a];
        let segment_b = segments_b[index_b];

        if !segment_a.is_orbit() {
            index_a += 1;
            continue;
        }

        if !segment_b.is_orbit() {
            index_a += 1;
            continue;
        }

        if segment_a.get_parent() == segment_b.get_parent() {
            same_parent_orbit_pairs.push((segment_a, segment_b));
        }

        if segment_a.get_end_time() < segment_b.get_end_time() {
            index_a += 1;
        } else {
            index_b += 1;
        }
    }

    same_parent_orbit_pairs
}

/// Step time needs to be large enough to be performant but small 
/// enough to catch all approaches. We choose this by choosing the 
/// minimum of the durations, and if applicable, periods, then dividing 
/// by a constant
fn get_time_step(segment_a: &Segment, segment_b: &Segment, start_time: f64, end_time: f64) -> f64 {
    let mut step_time = end_time - start_time;
    if let Segment::Orbit(orbit) = segment_a {
        if let Some(period) = orbit.get_period() {
            step_time = f64::min(step_time, period);
        }
    }
    if let Segment::Orbit(orbit) = segment_b {
        if let Some(period) = orbit.get_period() {
            step_time = f64::min(step_time, period);
        }
    }
    step_time / 16.0
}
impl Model {
    /// Returns the time at which the next closest approach will occur.
    /// This ignore burns. Why? Picture two spacecraft getting closer
    /// to each other. One of them starts burning to accelerate and
    /// ends up moving away from the other spacecraft. This is logical
    /// and makes sense, but in practice is very counterintuitive and
    /// isn't really useful information eg when trying to plan a
    /// rendezvous.
    pub fn find_next_closest_approach(&self, entity_a: Entity, entity_b: Entity, start_time: f64) -> Option<f64> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Find next closest approach");
        let same_parent_orbit_pairs = find_same_parent_orbit_pairs(self, entity_a, entity_b);

        for pair in same_parent_orbit_pairs {
            let segment_a = pair.0;
            let segment_b = pair.1;
            let start_time = f64::max(f64::max(segment_b.get_start_time(), segment_a.get_start_time()), start_time);
            let end_time = f64::min(segment_a.get_end_time(), segment_b.get_end_time());
            let time_step = get_time_step(segment_a, segment_b, start_time, end_time);
            let distance = |time: f64| (segment_a.get_position_at_time(time) - segment_b.get_position_at_time(time)).magnitude();
            let distance_prime = |time: f64| (distance(time + 2.0) - distance(time)) / 2.0;

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
                    return Some(itp(&distance_prime, previous_time, time));
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
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::{orbitable_component::OrbitableComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, storage::entity_builder::EntityBuilder, Model};

    use super::find_same_parent_orbit_pairs;

    #[test]
    fn test_find_same_parent_orbit_pairs() {
        let mut model = Model::default();

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3);
        let entity_builder = EntityBuilder::default();
        let entity_a = model.allocate(entity_builder.with_orbitable_component(orbitable));

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3);
        let entity_builder = EntityBuilder::default();
        let entity_b = model.allocate(entity_builder.with_orbitable_component(orbitable));

        let orbitable = OrbitableComponent::new(1.0e23, 1.0e3);
        let entity_builder = EntityBuilder::default();
        let entity_c = model.allocate(entity_builder.with_orbitable_component(orbitable));

        
        let mut trajectory = TrajectoryComponent::default();
    
        let mut orbit = Orbit::new(entity_c, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 0.0);
        orbit.end_at(10.0);
        let segment_d_1 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_d_1.clone());

        let mut orbit = Orbit::new(entity_b, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 10.0);
        orbit.end_at(50.0);
        let segment_d_2 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_d_2.clone());

        let mut orbit = Orbit::new(entity_c, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 50.0);
        orbit.end_at(100.0);
        let segment_d_3 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_d_3.clone());

        let entity_d = model.allocate(EntityBuilder::default().with_trajectory_component(trajectory));


        let mut trajectory = TrajectoryComponent::default();

        let mut orbit = Orbit::new(entity_a, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 0.0);
        orbit.end_at(5.0);
        let segment_e_1 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_e_1.clone());

        let mut orbit = Orbit::new(entity_b, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 5.0);
        orbit.end_at(15.0);
        let segment_e_2 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_e_2.clone());

        let mut orbit = Orbit::new(entity_c, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 15.0);
        orbit.end_at(55.0);
        let segment_e_3 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_e_3.clone());

        let mut orbit = Orbit::new(entity_a, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 55.0);
        orbit.end_at(70.0);
        let segment_e_4 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_e_4.clone());

        let mut orbit = Orbit::new(entity_c, 1.0e3, 1.0e23, vec2(1.0e9, 0.0), vec2(0.0, 1.0e3), 70.0);
        orbit.end_at(100.0);
        let segment_e_5 = Segment::Orbit(orbit);
        trajectory.add_segment(segment_e_5.clone());

        let entity_e = model.allocate(EntityBuilder::default().with_trajectory_component(trajectory));


        let expected = vec![
            (segment_d_2, segment_e_2),
            (segment_d_3.clone(), segment_e_3),
            (segment_d_3, segment_e_5),
        ];

        let actual = find_same_parent_orbit_pairs(&model, entity_d, entity_e);

        assert_eq!(actual.len(), expected.len());

        for i in 0..actual.len() {
            assert!(expected[i].0.get_parent() == actual[i].0.get_parent());
            assert!(expected[i].1.get_parent() == actual[i].1.get_parent());
        }
    }
}