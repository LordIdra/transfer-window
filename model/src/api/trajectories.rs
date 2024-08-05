use encounter::EncounterType;
use fast_solver::{calculate_entrance_encounter, calculate_exit_encounter};
use log::{error, trace};

use crate::{components::path_component::{burn::rocket_equation_function::RocketEquationFunction, orbit::{builder::OrbitBuilder, Orbit}, segment::Segment}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

use self::fast_solver::{apply_encounter, solver::find_next_encounter};

mod encounter;
mod fast_solver;
#[cfg(test)]
mod test_cases;

impl Model {
    /// Adds segments for all encounters after `entity`'s final
    /// segment but before `end_time`. Obviously, `entity` is
    /// expected to have a path component. Additionally,
    /// the final segment of entity's path MUST be an Orbit.
    /// Trajectory prediction is extremely complex, good luck if
    /// you need to modify this...
    /// Returns true is a new segment was added
    /// # Panics
    /// Panics if the last segment of the entity is a burn
    fn predict(&mut self, entity: Entity, end_time: f64, segment_count: usize) -> bool {
        if segment_count == 0 {
            return false;
        }
        
        let mut segments = 0;

        // A spacecraft that is for example in LEO will never have any
        // encounters, but without this, the predictor would keep on going every single frame
        // and rediscovering the exact same orbits
        if self.path_component(entity).final_orbit().unwrap().end_point().time() == end_time {
            return false;
        }

        loop {
            match find_next_encounter(self, self.path_component(entity).final_orbit().unwrap(), entity, end_time) {
                Ok(encounter) => {
                    if let Some(encounter) = encounter {
                        trace!("Found encounter {encounter:?}");
                        apply_encounter(self, &encounter);
                        segments += 1;
                        if segments >= segment_count {
                            break;
                        }
                    } else {
                        break;
                    }
                },
                Err(err) => {
                    error!("Error while predicting: {}", err);
                    break;
                },
            }
        }
        
        if segments < segment_count {
            self.path_component_mut(entity)
                .final_segment_mut()
                .as_orbit_mut()
                .expect("Attempt to predict when the last segment is a burn!")
                .end_at(end_time);
        }
        true
    }

    pub fn recompute_trajectory(&mut self, entity: Entity) -> bool {
        // Add 1 because the final orbit will have duration 0
        let segments_to_predict = SEGMENTS_TO_PREDICT + 1 - self.path_component(entity).future_orbits_after_final_non_orbit().len();
        self.predict(entity, 1.0e10, segments_to_predict)
    }

    pub fn recompute_entire_trajectory(&mut self, entity: Entity) {
        let current_segment = self.path_component(entity).current_segment();
        let parent = current_segment.parent();
        let orbit = OrbitBuilder {
            parent,
            mass: self.vessel_component(entity).mass(),
            parent_mass: self.mass(parent),
            rotation: current_segment.current_rotation(),
            position: current_segment.current_position(),
            velocity: current_segment.current_velocity(),
            time: self.time,
        }.build();

        self.path_component_mut(entity).clear_future_segments();
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn next_orbit(&self, entity: Entity, orbit: &mut Orbit) -> Option<Orbit> {
        match find_next_encounter(self, orbit, entity, 1.0e10) {
            Ok(encounter) => {
                if let Some(encounter) = encounter {
                    orbit.end_at(encounter.time());
                    Some(match encounter.type_() {
                        EncounterType::Entrance => calculate_entrance_encounter(self, orbit, encounter.new_parent(), encounter.time()),
                        EncounterType::Exit => calculate_exit_encounter(self, orbit, encounter.new_parent(), encounter.time()),
                    })
                } else {
                    orbit.end_at(1.0e10);
                    None
                }
            },
            Err(err) => {
                error!("Error while computing next orbit: {}", err);
                None
            },
        }
    }

    pub(crate) fn compute_perceived_path(&self, entity: Entity) -> Vec<Segment> {
        let current_segment = self.path_component(entity).current_segment();
        let parent = current_segment.parent();
        let orbit = OrbitBuilder {
            parent,
            mass: current_segment.current_mass(),
            parent_mass: self.mass(parent),
            rotation: current_segment.current_rotation(),
            position: current_segment.current_position(),
            velocity: current_segment.current_velocity(),
            time: self.time,
        }.build();
        
        let mut segments = Vec::new();
        segments.push(Segment::Orbit(orbit));

        while segments.len() < SEGMENTS_TO_PREDICT + 1 {
            let last_orbit = segments.last_mut().unwrap().as_orbit_mut().unwrap();
            let Some(orbit) = self.next_orbit(entity, last_orbit) else {
                break;
            };
            segments.push(Segment::Orbit(orbit));
        }

        segments
    }

    pub(crate) fn rocket_equation_function_at_end_of_trajectory(&self, entity: Entity) -> RocketEquationFunction {
        if let Some(rocket_equation_function) = self.path_component(entity).final_rocket_equation_function() {
            return rocket_equation_function;
        }

        RocketEquationFunction::fuel_from_vessel_component(self.vessel_component(entity))
    }
}
