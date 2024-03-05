use self::segment::Segment;

#[cfg(test)]
mod brute_force_tester;
pub mod burn;
pub mod orbit;
pub mod segment;

/// Must have MassComponent, cannot have StationaryComponent
#[derive(Debug)]
pub struct TrajectoryComponent {
    current_index: usize,
    segments: Vec<Option<Segment>>,
}

impl TrajectoryComponent {
    pub fn new() -> Self {
        Self { current_index: 0, segments: vec![] }
    }
}

impl TrajectoryComponent {
    pub fn get_segment_at_time(&self, time: f64) -> &Segment {
        for segment in &self.segments {
            if let Some(segment) = segment {
                if segment.get_start_time() <= time && segment.get_end_time() >= time {
                    return segment
                }
            }
        }
        panic!("No segment exists at the given time")
    }

    pub fn get_current_segment(&self) -> &Segment {
        self.segments
            .get(self.current_index as usize)
            .expect("Current segment does not exist")
            .as_ref()
            .unwrap() // current segment value should never be None
    }

    pub fn get_end_segment(&self) -> &Segment {
        self.segments
            .last()
            .expect("End segment does not exist")
            .as_ref()
            .unwrap() // end segment value should never be None
    }

    pub fn get_end_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .last_mut()
            .expect("End segment does not exist")
            .as_mut()
            .unwrap() // end segment value should never be None
    }

    fn get_current_segment_mut(&mut self) -> &mut Segment {
        self.segments
            .get_mut(self.current_index as usize)
            .expect("Current segment does not exist")
            .as_mut()
            .unwrap() // current segment value should never be None
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(Some(segment));
    }

    pub fn update(&mut self, delta_time: f64) {
        let current_segment = self.get_current_segment_mut();
        current_segment.update(delta_time);
        if current_segment.is_finished() {
            self.current_index += 1;
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
        let mut trajectory = TrajectoryComponent::new();

        let orbit_1 = {
            let parent = Entity::mock();
            let mass = 100.0;
            let parent_mass = 5.9722e24;
            let position = vec2(2.0e6, 0.0);
            let velocity = vec2(0.0, 1.0e5);
            let start_time = 0.0;
            let end_time = 100.0;
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
            let start_time = 100.0;
            let end_time = 200.0;
            let mut orbit = Orbit::new(parent, mass, parent_mass, position, velocity, start_time);
            orbit.end_at(end_time);
            orbit
        };

        trajectory.add_segment(Segment::Orbit(orbit_1));
        trajectory.add_segment(Segment::Orbit(orbit_2));

        assert!(trajectory.current_index == 0);
        assert!(trajectory.get_segment_at_time(105.0).get_start_time() == 100.0);

        let end_position_1 = trajectory.get_segment_at_time(43.64).get_end_position();
        let end_position_2 = trajectory.get_segment_at_time(172.01).get_end_position();
        assert!((end_position_1.magnitude() - end_position_2.magnitude()) < 1.0e-3);

        for _ in 0..200 {
            trajectory.update(1.0);
        }

        assert!(trajectory.current_index == 1);
    }
}