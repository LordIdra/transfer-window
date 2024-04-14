use log::{error, trace};
use serde::{Deserialize, Serialize};

use self::{burn::Burn, segment::Segment};

#[cfg(test)]
mod brute_force_tester;
pub mod burn;
pub mod orbit;
pub mod segment;

/// Must have `MassComponent`, cannot have `StationaryComponent`
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrajectoryComponent {
    current_index: usize,
    previous_orbits: usize,
    previous_burns: usize,
    segments: Vec<Option<Segment>>,
}

impl TrajectoryComponent {
    pub fn get_segments(&self) -> &Vec<Option<Segment>> {
        &self.segments
    }

    pub fn get_previous_orbits(&self) -> usize {
        self.previous_orbits
    }

    pub fn get_previous_burns(&self) -> usize {
        self.previous_burns
    }

    pub fn get_remaining_orbits(&self) -> usize {
        self.segments.iter()
            .flatten()
            .filter(|segment| matches!(segment, Segment::Orbit(_)))
            .count()
    }

    pub fn get_remaining_burns(&self) -> usize {
        self.segments.iter()
            .flatten()
            .filter(|segment| matches!(segment, Segment::Burn(_)))
            .count()
    }

    pub fn get_remaining_orbits_after_final_burn(&self) -> usize {
        let mut remaining_orbits = 0;
        for segment in self.segments.iter().rev().flatten() {
            match segment {
                Segment::Burn(_) => break,
                Segment::Orbit(_) => remaining_orbits += 1,
            }
        }
        remaining_orbits
    }

    pub fn get_final_burn(&self) -> Option<&Burn> {
        for segment in self.segments.iter().rev().flatten() {
            if let Segment::Burn(burn) = segment {
                return Some(burn)
            }
        }
        None
    }

    /// Returns the first segment it finds matching the time
    /// If the time is exactly on the border between two segments,
    /// returns the first one
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn get_first_segment_at_time(&self, time: f64) -> &Segment {
        for segment in self.segments.iter().flatten() {
            if segment.get_start_time() <= time && segment.get_end_time() >= time {
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// Returns the first segment it finds matching the time
    /// If the time is exactly on the border between two segments,
    /// returns the last one
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn get_last_segment_at_time(&self, time: f64) -> &Segment {
        for segment in self.segments.iter().flatten() {
            if segment.get_start_time() <= time && segment.get_end_time() > time {
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn get_last_segment_at_time_mut(&mut self, time: f64) -> &mut Segment {
        for segment in self.segments.iter_mut().flatten() {
            if segment.get_start_time() <= time && segment.get_end_time() > time {
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

    /// # Panics
    /// Panics if the segment at `time` is a burn
    pub fn remove_segments_after(&mut self, time: f64) {
        loop {
            match self.segments.last_mut().unwrap().as_mut().unwrap() {
                Segment::Burn(burn) => {
                    // The >= is important, because we might try and remove segments after exactly the start time of a burn (ie when deleting a burn)
                    if burn.get_start_point().get_time() >= time {
                        self.segments.pop();
                    } else if burn.is_time_within_burn(time) {
                        error!("Attempt to split a burn");
                        panic!("Error recoverable, but exiting anyway before something bad happens");
                    } else {
                        return;
                    }
                },
                Segment::Orbit(orbit) => {
                    if orbit.get_start_point().get_time() >= time {
                        self.segments.pop();
                    } else if orbit.is_time_within_orbit(time) {
                        orbit.end_at(time);
                    } else {
                        return;
                    }
                },
            }
        }
    }

    pub fn next(&mut self, time: f64, delta_time: f64) {
        self.get_current_segment_mut().next(delta_time);
        while self.get_current_segment().is_finished() {
            match self.get_current_segment() {
                Segment::Orbit(_) => self.previous_orbits += 1,
                Segment::Burn(_) => self.previous_burns += 1,
            }
            trace!("Segment finished at time={time}");
            let overshot_time = self.get_current_segment().get_overshot_time(time);
            self.segments[self.current_index] = None;
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
        assert!(trajectory.get_first_segment_at_time(105.0).get_start_time() == 99.9);

        let end_position_1 = trajectory.get_first_segment_at_time(43.65).get_end_position();
        let end_position_2 = trajectory.get_first_segment_at_time(172.01).get_end_position();
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