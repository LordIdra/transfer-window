use serde::{Deserialize, Serialize};

use self::segment::Segment;

#[cfg(test)]
mod brute_force_tester;
pub mod burn;
pub mod orbit;
pub mod segment;

/// Must have `MassComponent`, cannot have `StationaryComponent`
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrajectoryComponent {
    current_index: usize,
    segments: Vec<Option<Segment>>,
}

impl TrajectoryComponent {
    pub fn get_segments(&self) -> &Vec<Option<Segment>> {
        &self.segments
    }

    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn get_segment_at_time(&self, time: f64) -> &Segment {
        for segment in self.segments.iter().flatten() {
            if segment.get_start_time() <= time && segment.get_end_time() >= time {
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// # Panics
    /// Panics if the trajectory has no current segment
    pub fn get_current_segment(&self) -> &Segment {
        self.segments
            .get(self.current_index)
            .expect("Current segment does not exist")
            .as_ref()
            .unwrap() // current segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no end segment
    pub fn get_end_segment(&self) -> &Segment {
        self.segments
            .last()
            .expect("End segment does not exist")
            .as_ref()
            .unwrap() // end segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no start segment
    pub fn get_end_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .last_mut()
            .expect("End segment does not exist")
            .as_mut()
            .unwrap() // end segment value should never be None
    }

    fn get_current_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .get_mut(self.current_index)
            .expect("Current segment does not exist")
            .as_mut()
            .unwrap() // current segment value should never be None
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(Some(segment));
    }

    pub fn next(&mut self, time: f64, delta_time: f64) {
        self.get_current_segment_mut().next(delta_time);
        while self.get_current_segment().is_finished() {
            let overshot_time = self.get_current_segment().get_overshot_time(time);
            self.current_index += 1;
            self.get_current_segment_mut().next(overshot_time);
        }
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{storage::entity_allocator::Entity, components::trajectory_component::TrajectoryComponent};

    use super::{segment::Segment, orbit::Orbit};


    #[test]
    #[allow(clippy::float_cmp)]
    pub fn test() {
        let mut trajectory = TrajectoryComponent::default();

        let orbit_1 = {
            let parent = Entity::mock();
            let mass = 100.0;
            let parent_mass = 5.9722e24;
            let position = vec2(2.0e6, 0.0);
            let velocity = vec2(0.0, 1.0e5);
            let start_time = 0.0;
            let end_time = 99.9;
            let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
            orbit.end_at(end_time);
            orbit
        };

        let orbit_2 = {
            // Exact same start point but opposite velocity, so same end velocity/position magnitude
            let parent = Entity::mock();
            let mass = 100.0;
            let parent_mass = 5.9722e24;
            let position = vec2(2.0e6, 0.0);
            let velocity = vec2(0.0, -1.0e5); 
            let start_time = 99.9;
            let end_time = 200.0;
            let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
            orbit.end_at(end_time);
            orbit
        };

        trajectory.add_segment(Segment::Orbit(orbit_1));
        trajectory.add_segment(Segment::Orbit(orbit_2));

        assert!(trajectory.current_index == 0);
        assert!(trajectory.get_segment_at_time(105.0).get_start_time() == 99.9);

        let end_position_1 = trajectory.get_segment_at_time(43.65).get_end_position();
        let end_position_2 = trajectory.get_segment_at_time(172.01).get_end_position();
        let m1 = end_position_1.magnitude();
        let m2 = end_position_2.magnitude();
        let difference = (m1 - m2) / m1;
        assert!(difference < 1.0e-4);

        let mut time = 0.0;
        for _ in 0..100 {
            time += 1.0;
            trajectory.next(time, 1.0);
        }

        assert!((trajectory.get_current_segment().as_orbit().get_current_point().get_time() - 100.0).abs() < 1.0e-6);
        assert!(trajectory.current_index == 1);
    }
}