use nalgebra_glm::{vec2, DVec2};

use crate::{components::path_component::{burn::{builder::BurnBuilder, rocket_equation_function::RocketEquationFunction, Burn}, orbit::builder::OrbitBuilder, segment::Segment}, storage::entity_allocator::Entity, Model};

const MIN_DV_TO_ADJUST_BURN: f64 = 1.0e-2;

impl Model {
    pub(crate) fn create_burn(&mut self, entity: Entity, time: f64, rocket_equation_function: RocketEquationFunction) {
        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).final_segment();
        let parent = last_segment.parent();
        let parent_mass = self.mass(parent);

        let burn = BurnBuilder {
            parent,
            parent_mass,
            rocket_equation_function,
            tangent: last_segment.end_velocity().normalize(),
            delta_v: vec2(0.0, 0.0),
            time,
            position: last_segment.end_position(),
            velocity: last_segment.end_velocity(),
        }.build();

        let end_point = burn.end_point();

        let orbit = OrbitBuilder {
            parent,
            mass: end_point.mass(),
            parent_mass,
            rotation: burn.rotation(),
            position: end_point.position(),
            velocity: end_point.velocity(),
            time: end_point.time()
        }.build();

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
        let velocity = burn.end_point().velocity();

        let orbit = OrbitBuilder {
            parent: burn.parent(),
            mass: burn.end_point().mass(),
            parent_mass: self.mass(parent),
            rotation: f64::atan2(velocity.y, velocity.x),
            position: burn.end_point().position(),
            velocity,
            time: burn.end_point().time(),
        }.build();

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
        if new_dv > burn.start_rocket_equation_function().remaining_dv() {
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
