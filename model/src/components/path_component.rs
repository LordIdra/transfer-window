use std::collections::VecDeque;

use log::trace;
use serde::{Deserialize, Serialize};
use turn::Turn;

use self::{burn::Burn, guidance::Guidance, orbit::Orbit, segment::Segment};

#[cfg(test)]
mod brute_force_tester;
pub mod burn;
pub mod guidance;
pub mod orbit;
pub mod rocket_equation_function;
pub mod segment;
pub mod turn;

/// Must have `MassComponent`, cannot have `StationaryComponent`
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PathComponent {
    past_segments: Vec<Segment>,
    future_segments: VecDeque<Segment>,
    /// The segments a faction without intel on this entity would see
    perceived_segments: Vec<Segment>,
}

impl PathComponent {
    pub fn new_with_segment(segment: Segment) -> Self {
        Self::default().with_segment(segment)
    }

    pub fn new_with_orbit(orbit: Orbit) -> Self {
        Self::default().with_segment(Segment::Orbit(orbit))
    }

    pub fn new_with_burn(burn: Burn) -> Self {
        Self::default().with_segment(Segment::Burn(burn))
    }

    pub fn past_segments(&self) -> Vec<&Segment> {
        self.past_segments.iter().collect()
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

    pub fn future_segments(&self) -> Vec<&Segment> {
        self.future_segments.iter().collect()
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

    pub fn future_turns(&self) -> Vec<&Turn> {
        self.future_segments.iter()
            .filter_map(|segment| segment.as_turn())
            .collect()
    }
    
    pub fn future_guidances(&self) -> Vec<&Guidance> {
        self.future_segments.iter()
            .filter_map(|segment| segment.as_guidance())
            .collect()
    }

    pub fn future_orbits_after_last_non_orbit(&self) -> Vec<&Orbit> {
        let mut orbits = vec![];
        for segment in self.future_segments.iter().rev() {
            match segment {
                Segment::Orbit(orbit) => orbits.push(orbit),
                Segment::Burn(_) | Segment::Guidance(_) | Segment::Turn(_) => break,
            }
        }
        orbits
    }

    pub fn end_burn(&self) -> Option<&Burn> {
        self.future_burns().last().copied()
    }
    
    pub fn end_orbit(&self) -> Option<&Orbit> {
        self.future_orbits().last().copied()
    }
    
    pub fn end_guidance(&self) -> Option<&Guidance> {
        self.future_guidances().last().copied()
    }

    pub fn end_dv(&self) -> Option<f64> {
        for segment in self.future_segments.iter().rev() {
            if let Segment::Burn(burn) = segment {
                return Some(burn.end_dv());
            }
            if let Segment::Guidance(guidance) = segment {
                return Some(guidance.end_dv());
            }
        }
        None
    }

    pub fn end_fuel_kg(&self) -> Option<f64> {
        for segment in self.future_segments.iter().rev() {
            if let Segment::Burn(burn) = segment {
                return Some(burn.end_fuel_kg());
            }
            if let Segment::Guidance(guidance) = segment {
                return Some(guidance.end_fuel_kg());
            }
        }
        None
    }

    pub fn fuel_kg_at_time(&self, time: f64) -> Option<f64> {
        if let Segment::Burn(burn) = self.future_segment_at_time(time) {
            return Some(burn.fuel_kg_at_time(time));
        }
        if let Segment::Guidance(guidance) = self.future_segment_at_time(time) {
            return Some(guidance.fuel_kg_at_time(time));
        }
        if let Segment::Turn(turn) = self.future_segment_at_time(time) {
            return Some(turn.fuel_kg_at_time(time));
        }

        for segment in self.future_segments.iter().rev() {
            if segment.start_time() > time {
                continue;
            }
            if let Segment::Burn(burn) = segment {
                return Some(burn.end_fuel_kg());
            }
            if let Segment::Guidance(guidance) = segment {
                return Some(guidance.end_fuel_kg());
            }
            if let Segment::Turn(turn) = segment {
                return Some(turn.end_fuel_kg());
            }
        }

        None
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
    pub fn end_segment(&self) -> &Segment {
        self.future_segments
            .back()
            .as_ref()
            .unwrap() // end segment value should never be None
    }

    /// # Panics
    /// Panics if the trajectory has no start segment
    pub fn end_segment_mut(&mut self) -> &mut Segment {
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
            Segment::Guidance(guidance) => guidance.current_point().mass(),
            Segment::Turn(turn) => turn.current_point().mass(),
        }
    }

    pub fn mass_at_time(&self, time: f64) -> f64 {
        match self.future_segment_at_time(time) {
            Segment::Orbit(orbit) => orbit.mass(),
            Segment::Burn(burn) => burn.point_at_time(time).mass(),
            Segment::Guidance(guidance) => guidance.point_at_time(time).mass(),
            Segment::Turn(turn) => turn.point_at_time(time).mass(),
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
    /// Panics if the segment at `time` is a burn or guidance segment
    pub fn remove_segments_after(&mut self, time: f64) {
        loop {
            match self.future_segments.back_mut().as_mut().unwrap() {
                Segment::Burn(burn) => {
                    // The >= is important, because we might try and remove segments after exactly the start time of a burn (ie when deleting a burn)
                    if burn.start_point().time() >= time {
                        self.future_segments.pop_back();
                    } else if burn.is_time_within_burn(time) {
                        panic!("Attempt to split a burn");
                    } else {
                        return;
                    }
                },

                Segment::Guidance(guidance) => {
                    if guidance.start_point().time() >= time {
                        self.future_segments.pop_back();
                    } else if guidance.is_time_within_guidance(time) {
                        panic!("Attempt to split a guidance segment");
                    } else {
                        return;
                    }
                },

                Segment::Turn(turn) => {
                    if turn.start_point().time() >= time {
                        self.future_segments.pop_back();
                    } else if turn.is_time_within_turn(time) {
                        panic!("Attempt to split a turn segment");
                    } else {
                        return;
                    }
                }

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

    pub fn remove_last_segment(&mut self) {
        self.future_segments.pop_back();
    }

    /// # Panics
    /// Panics if there are no more future segments after the finished segment
    pub fn on_segment_finished(&mut self, time: f64) {
        trace!("Segment finished at time={time}");
        self.past_segments.push(self.future_segments.pop_front().expect("No more future segments"));
        self.current_segment_mut().next(time);
    }

    pub fn clear_future_segments(&mut self) {
        self.future_segments.clear();
    }
    
    pub fn perceived_segments(&self) -> Vec<&Segment> {
        self.perceived_segments.iter().collect()
    }

    pub fn current_perceived_segment(&self) -> &Segment {
        self.perceived_segments.first().unwrap()
    }

    pub fn current_perceived_segment_mut(&mut self) -> &mut Segment {
        self.perceived_segments.first_mut().unwrap()
    }

    /// Returns the first segment it finds matching the time
    /// If the time is exactly on the border between two segments,
    /// returns the first one
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn perceived_segment_at_time(&self, time: f64) -> &Segment {
        for segment in &self.perceived_segments {
            // The reason we clamp to the end time is because encounter prediction is nondeterministic
            // So if we call encounter prediction one frame, get the time, and store the result, the
            // same call the next frame might be very slightly sooner. Then suppose we feed the result into
            // this function... oh, look, we got a panic because the stored time is slightly after the 
            // end segment. Yes this is a stupid solution but I don't know how to better solve it
            let time = f64::min(time, self.perceived_segments.last().unwrap().end_time());
            if time >= segment.start_time() && time <= segment.end_time(){
                return segment
            }
        }
        panic!("No segment exists at the given time")
    }

    /// Returns the first segment it finds exactly matching the start time
    /// # Panics
    /// Panics if the trajectory has no segment at the given time
    pub fn perceived_segment_starting_at_time(&self, time: f64) -> Option<&Segment> {
        for segment in &self.perceived_segments {
            if time == segment.start_time() {
                return Some(segment)
            }
        }
        None
    }

    pub fn set_perceived_segments(&mut self, perceived_segments: Vec<Segment>) {
        self.perceived_segments = perceived_segments;
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::path_component::{orbit::builder::OrbitBuilder, PathComponent}, storage::entity_allocator::Entity};

    use super::segment::Segment;


    #[test]
    pub fn test() {
        let mut path_component = PathComponent::default();

        let orbit_1 = OrbitBuilder {
            parent: Entity::mock(),
            mass: 100.0,
            parent_mass: 5.9722e24,
            rotation: 0.0,
            position: vec2(2.0e6, 0.0),
            velocity: vec2(0.0, 1.0e5),
            time: 0.0,
        }.build().with_end_at(99.0);

        let orbit_2 = OrbitBuilder {
            // Exact same start point but opposite velocity, so same end velocity/position magnitude
            parent: Entity::mock(),
            mass: 100.0,
            parent_mass: 5.9722e24,
            rotation: 0.0,
            position: vec2(2.0e6, 0.0),
            velocity: vec2(0.0, -1.0e5),
            time: 99.9,
        }.build().with_end_at(200.0);

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
            path_component.current_segment_mut().next(time);
            while path_component.current_segment().is_finished() {
                path_component.on_segment_finished(time);
            }
        }

        assert!((path_component.current_segment().as_orbit().unwrap().current_point().time() - 100.0).abs() < 1.0e-6);
        assert!(path_component.past_segments().len() == 1);
    }
}
