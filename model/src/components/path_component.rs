use std::collections::VecDeque;

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
    past_segments: Vec<Segment>,
    future_segments: VecDeque<Segment>,
}

impl PathComponent {
    pub fn new_with_segment(segment: Segment) -> Self {
        Self::default().with_segment(segment)
    }

    pub fn new_with_orbit(orbit: Orbit) -> Self {
        Self::default().with_segment(Segment::Orbit(orbit))
    }

    pub fn past_segments(&self) -> &Vec<Segment> {
        &self.past_segments
    }

    pub fn past_orbits(&self) -> Vec<&Orbit> {
        self.past_segments.iter()
            .filter_map(|segment| segment.as_orbit())
            .collect()
    }

    pub fn past_burns(&self) -> Vec<&Burn> {
        self.past_segments.iter()
            .filter_map(|segment| segment.as_burn())
            .collect()
    }

    pub fn future_segments(&self) -> &VecDeque<Segment> {
        &self.future_segments
    }

    pub fn future_orbits(&self) -> Vec<&Orbit> {
        self.future_segments.iter()
            .filter_map(|segment| segment.as_orbit())
            .collect()
    }

    pub fn future_burns(&self) -> Vec<&Burn> {
        self.future_segments.iter()
            .filter_map(|segment| segment.as_burn())
            .collect()
    }

    pub fn future_orbits_after_final_burn(&self) -> Vec<&Orbit> {
        let mut orbits = vec![];
        for segment in self.future_segments.iter().rev() {
            match segment {
                Segment::Burn(_) => break,
                Segment::Orbit(orbit) => orbits.push(orbit),
            }
        }
        orbits
    }

    pub fn final_burn(&self) -> Option<&Burn> {
        self.future_burns().last().copied()
    }

    pub fn final_orbit(&self) -> Option<&Orbit> {
        self.future_orbits().last().copied()
    }

    /// Returns the first segment it finds matching the time
    /// If the time is exactly on the border between two segments,
    /// returns the first one
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn future_segment_at_time(&self, time: f64) -> &Segment {
        for segment in &self.future_segments {
            if time >= segment.start_time() && time <= segment.end_time(){
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// Returns the first segment it finds exactly matching the start time
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn future_segment_starting_at_time(&self, time: f64) -> Option<&Segment> {
        for segment in &self.future_segments {
            if time == segment.start_time() {
                return Some(segment)
            }
        }
        None
    }

    /// # Panics
    /// Panics if the trajectory has no current segment
    pub fn current_segment(&self) -> &Segment {
        self.future_segments
            .front()
            .as_ref()
            .unwrap() // current segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no end segment
    pub fn last_segment(&self) -> &Segment {
        self.future_segments
            .back()
            .as_ref()
            .unwrap() // end segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no start segment
    pub fn last_segment_mut(&mut self) -> &mut Segment {
        self.future_segments
            .back_mut()
            .unwrap()
    }

    /// # Panics
    /// Panics if the trajectory has no current segment
    pub fn current_segment_mut(&mut self) -> &mut Segment {
        self.future_segments
            .front_mut()
            .unwrap()
    }

    pub fn current_mass(&self) -> f64 {
        match self.current_segment() {
            Segment::Orbit(orbit) => orbit.mass(),
            Segment::Burn(burn) => burn.current_point().mass(),
        }
    }

    pub fn mass_at_time(&self, time: f64) -> f64 {
        match self.future_segment_at_time(time) {
            Segment::Orbit(orbit) => orbit.mass(),
            Segment::Burn(burn) => burn.current_point().mass(),
        }
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.future_segments.push_back(segment);
    }

    pub fn with_segment(mut self, segment: Segment) -> Self {
        self.future_segments.push_back(segment);
        self
    }

    /// # Panics
    /// Panics if the segment at `time` is a burn
    pub fn remove_segments_after(&mut self, time: f64) {
        loop {
            match self.future_segments.back_mut().as_mut().unwrap() {
                Segment::Burn(burn) => {
                    // The >= is important, because we might try and remove segments after exactly the start time of a burn (ie when deleting a burn)
                    if burn.start_point().time() >= time {
                        self.future_segments.pop_back();
                    } else if burn.is_time_within_burn(time) {
                        error!("Attempt to split a burn");
                        panic!("Error recoverable, but exiting anyway before something bad happens");
                    } else {
                        return;
                    }
                },
                Segment::Orbit(orbit) => {
                    if orbit.start_point().time() >= time {
                        self.future_segments.pop_back();
                    } else if orbit.is_time_within_orbit(time) {
                        orbit.end_at(time);
                    } else {
                        return;
                    }
                },
            }
        }
    }

    /// # Panics
    /// Panics if there are no more future segments after the finished segment
    pub fn on_segment_finished(&mut self, time: f64) {
        trace!("Segment finished at time={time}");
        let overshot_time = self.current_segment().overshot_time(time);
        self.past_segments.push(self.future_segments.pop_front().expect("No more future segments"));
        self.current_segment_mut().next(overshot_time);
    }

    pub fn clear_future_segments(&mut self) {
        self.future_segments.clear();
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

        assert!(path_component.past_segments().is_empty());
        assert!(path_component.future_segment_at_time(105.0).start_time() == 99.9);

        let end_position_1 = path_component.future_segment_at_time(43.65).end_position();
        let end_position_2 = path_component.future_segment_at_time(172.01).end_position();
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
        assert!(path_component.past_segments().len() == 1);
    }
}