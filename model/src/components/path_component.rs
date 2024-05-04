use log::{error, trace};
use serde::{Deserialize, Serialize};

use self::{burn::Burn, orbit::Orbit, segment::Segment};

#[cfg(test)]
mod brute_force_tester;
pub mod burn;
pub mod orbit;
pub mod segment;

/// Must have `MassComponent`, cannot have `StationaryComponent`
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PathComponent {
    current_index: usize,
    previous_orbits: usize,
    previous_burns: usize,
    segments: Vec<Option<Segment>>,
}

impl PathComponent {
    pub fn new_with_segment(segment: Segment) -> Self {
        Self::default().with_segment(segment)
    }

    pub fn new_with_orbit(orbit: Orbit) -> Self {
        Self::default().with_segment(Segment::Orbit(orbit))
    }

    pub fn segments(&self) -> &Vec<Option<Segment>> {
        &self.segments
    }

    pub fn previous_orbits(&self) -> usize {
        self.previous_orbits
    }

    pub fn previous_burns(&self) -> usize {
        self.previous_burns
    }

    pub fn remaining_orbits(&self) -> usize {
        self.segments.iter()
            .flatten()
            .filter(|segment| matches!(segment, Segment::Orbit(_)))
            .count()
    }

    pub fn remaining_burns(&self) -> usize {
        self.segments.iter()
            .flatten()
            .filter(|segment| matches!(segment, Segment::Burn(_)))
            .count()
    }

    pub fn remaining_orbits_after_final_burn(&self) -> usize {
        let mut remaining_orbits = 0;
        for segment in self.segments.iter().rev().flatten() {
            match segment {
                Segment::Burn(_) => break,
                Segment::Orbit(_) => remaining_orbits += 1,
            }
        }
        remaining_orbits
    }

    pub fn final_burn(&self) -> Option<&Burn> {
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
    pub fn first_segment_at_time(&self, time: f64) -> &Segment {
        for segment in self.segments.iter().flatten() {
            if segment.start_time() <= time && segment.end_time() >= time {
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
    pub fn last_segment_at_time(&self, time: f64) -> &Segment {
        for segment in self.segments.iter().flatten() {
            if segment.start_time() <= time && segment.end_time() > time {
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn last_segment_at_time_mut(&mut self, time: f64) -> &mut Segment {
        for segment in self.segments.iter_mut().flatten() {
            if segment.start_time() <= time && segment.end_time() > time {
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// # Panics
    /// Panics if the trajectory has no current segment
    pub fn current_segment(&self) -> &Segment {
        self.segments
            .get(self.current_index)
            .expect("Current segment does not exist")
            .as_ref()
            .unwrap() // current segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no end segment
    pub fn end_segment(&self) -> &Segment {
        self.segments
            .last()
            .expect("End segment does not exist")
            .as_ref()
            .unwrap() // end segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no start segment
    pub fn end_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .last_mut()
            .expect("End segment does not exist")
            .as_mut()
            .unwrap() // end segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no current segment
    pub fn current_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .get_mut(self.current_index)
            .expect("Current segment does not exist")
            .as_mut()
            .unwrap() // current segment value should never be None
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(Some(segment));
    }

    pub fn with_segment(mut self, segment: Segment) -> Self {
        self.segments.push(Some(segment));
        self
    }

    /// # Panics
    /// Panics if the segment at `time` is a burn
    pub fn remove_segments_after(&mut self, time: f64) {
        loop {
            match self.segments.last_mut().unwrap().as_mut().unwrap() {
                Segment::Burn(burn) => {
                    // The >= is important, because we might try and remove segments after exactly the start time of a burn (ie when deleting a burn)
                    if burn.start_point().time() >= time {
                        self.segments.pop();
                    } else if burn.is_time_within_burn(time) {
                        error!("Attempt to split a burn");
                        panic!("Error recoverable, but exiting anyway before something bad happens");
                    } else {
                        return;
                    }
                },
                Segment::Orbit(orbit) => {
                    if orbit.start_point().time() >= time {
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

    pub fn on_segment_finished(&mut self, time: f64) {
        match self.current_segment() {
            Segment::Orbit(_) => self.previous_orbits += 1,
            Segment::Burn(_) => self.previous_burns += 1,
        }
        trace!("Segment finished at time={time}");
        let overshot_time = self.current_segment().overshot_time(time);
        self.segments[self.current_index] = None;
        self.current_index += 1;
        self.current_segment_mut().next(overshot_time);
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{storage::entity_allocator::Entity, components::path_component::PathComponent};

    use super::{segment::Segment, orbit::Orbit};


    #[test]
    pub fn test() {
        let mut path_component = PathComponent::default();

        let orbit_1 = {
            let parent = Entity::mock();
            let mass = 100.0;
            let parent_mass = 5.9722e24;
            let position = vec2(2.0e6, 0.0);
            let velocity = vec2(0.0, 1.0e5);
            let start_time = 0.0;
            let end_time = 99.9;
            Orbit::new(parent, mass, parent_mass, position, velocity, start_time).with_end_at(end_time)
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
            Orbit::new(parent, mass, parent_mass, position, velocity, start_time).with_end_at(end_time)
        };

        path_component.add_segment(Segment::Orbit(orbit_1));
        path_component.add_segment(Segment::Orbit(orbit_2));

        assert!(path_component.current_index == 0);
        assert!(path_component.first_segment_at_time(105.0).start_time() == 99.9);

        let end_position_1 = path_component.first_segment_at_time(43.65).end_position();
        let end_position_2 = path_component.first_segment_at_time(172.01).end_position();
        let m1 = end_position_1.magnitude();
        let m2 = end_position_2.magnitude();
        let difference = (m1 - m2) / m1;
        assert!(difference < 1.0e-4);

        let mut time = 0.0;
        for _ in 0..100 {
            time += 1.0;
            path_component.current_segment_mut().next(1.0);
            while path_component.current_segment().is_finished() {
                path_component.on_segment_finished(time);
            }
        }

        assert!((path_component.current_segment().as_orbit().unwrap().current_point().time() - 100.0).abs() < 1.0e-6);
        assert!(path_component.current_index == 1);
    }
}