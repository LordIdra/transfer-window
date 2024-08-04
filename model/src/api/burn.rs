use nalgebra_glm::{vec2, DVec2};

use crate::{components::path_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, orbit::Orbit, segment::Segment}, storage::entity_allocator::Entity, Model};

const MIN_DV_TO_ADJUST_BURN: f64 = 1.0e-2;

impl Model {
    pub(crate) fn create_burn(&mut self, entity: Entity, time: f64, rocket_equation_function: RocketEquationFunction) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);

        let last_segment = path_component.final_segment();
        let parent = last_segment.parent();
        let tangent = last_segment.end_velocity().normalize();
        let start_position = last_segment.end_position();
        let start_velocity = last_segment.end_velocity();
        let parent_mass = self.mass(parent);
        let burn = Burn::new(parent, parent_mass, tangent, vec2(0.0, 0.0), time, rocket_equation_function, start_position, start_velocity);

        let end_point = burn.end_point();
        let orbit = Orbit::new(parent, end_point.mass(), parent_mass, end_point.position(), end_point.velocity(), end_point.time());

        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn delete_burn(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if there is no burn at the specified time
    pub(crate) fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let mut burn = self.burn_starting_at_time(entity, time).clone();
        burn.adjust(amount);
        
        let parent = burn.parent();
        let position = burn.end_point().position();
        let velocity = burn.end_point().velocity();
        let mass = burn.end_point().mass();
        let end_time = burn.end_point().time();
        let parent_mass = self.mass(parent);
        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, end_time);

        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(burn.start_point().time());
        path_component.add_segment(Segment::Burn(burn));
        path_component.add_segment(Segment::Orbit(orbit));

        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if the entity does not have a burn at the given time
    pub fn burn_starting_at_time(&self, entity: Entity, time: f64) -> &Burn {
        if let Some(path_component) = self.try_path_component(entity) {
            if let Some(Segment::Burn(burn)) = path_component.future_segment_starting_at_time(time) {
                return burn;
            }
        }

        panic!("There is no burn at the requested time")
    }

    pub fn calculate_burn_dv(&self, entity: Entity, time: f64, change: DVec2) -> Option<DVec2> {
        let burn = self.burn_starting_at_time(entity, time);
        let new_dv = (burn.delta_v() + change).magnitude();
        if new_dv > burn.rocket_equation_function().remaining_dv() {
            if burn.final_rocket_equation_function().remaining_dv() < MIN_DV_TO_ADJUST_BURN {
                None
            } else {
                Some(change.normalize() * burn.final_rocket_equation_function().remaining_dv() * 0.999)
            }
        } else {
            Some(change)
        }
    }
}
