use nalgebra_glm::{vec2, DVec2};

use crate::{components::{path_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, orbit::Orbit, segment::Segment}, vessel_component::system_slot::System}, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

impl Model {
    fn rocket_equation_function_at_end_of_trajectory(&self, entity: Entity) -> RocketEquationFunction {
        if let Some(burn) = self.path_component(entity).final_burn() {
            return burn.rocket_equation_function_at_end_of_burn()
        }

        let vessel_component = self.vessel_component(entity);
        let dry_mass_kg = vessel_component.dry_mass();
        let initial_fuel_mass_kg = vessel_component.remaining_fuel_kg();
        let engine = vessel_component.slots().engine().unwrap();
        let fuel_consumption_kg_per_second = engine.type_().fuel_kg_per_second();
        let specific_impulse = engine.type_().specific_impulse_space();
        RocketEquationFunction::new(dry_mass_kg, initial_fuel_mass_kg, fuel_consumption_kg_per_second, specific_impulse, 0.0)
    }

    pub fn delete_segments_after_time_and_recompute_trajectory(&mut self, entity: Entity, time: f64) {
        let trajectory_component = self.path_component_mut(entity);
        trajectory_component.remove_segments_after(time);

        // Recompute new trajectory
        // Add 1 because the final orbit will have duration 0
        let segments_to_predict = SEGMENTS_TO_PREDICT + 1 - trajectory_component.remaining_orbits_after_final_burn();
        self.predict(entity, 1.0e10, segments_to_predict);
    }

    pub fn create_burn(&mut self, entity: Entity, time: f64) {
        let trajectory_component = self.path_component_mut(entity);
        trajectory_component.remove_segments_after(time);

        let parent = trajectory_component.end_segment().parent();
        let tangent = trajectory_component.end_segment().end_velocity().normalize();
        let start_position = trajectory_component.end_segment().end_position();
        let start_velocity = trajectory_component.end_segment().end_velocity();
        let rocket_equation_function = self.rocket_equation_function_at_end_of_trajectory(entity);
        let parent_mass = self.mass(parent);
        let burn = Burn::new(entity, parent, parent_mass, tangent, vec2(0.0, 0.0), time, rocket_equation_function, start_position, start_velocity);

        let end_point = burn.end_point();
        let orbit = Orbit::new(parent, end_point.mass(), parent_mass, end_point.position(), end_point.velocity(), end_point.time());

        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.predict(entity, 1.0e10, SEGMENTS_TO_PREDICT);
    }

    pub fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let trajectory_component = self.path_component_mut(entity);
        let end_time = trajectory_component.last_segment_at_time(time).end_time();
        trajectory_component.remove_segments_after(end_time);
        trajectory_component.end_segment_mut()
            .as_burn_mut()
            .expect(&format!("Burn not found at time {}", time))
            .adjust(amount);

        let end_segment = trajectory_component.end_segment();
        let parent = end_segment.parent();
        let position = end_segment.end_position();
        let velocity = end_segment.end_velocity();
        let parent_mass = self.mass(parent);
        let mass = self.mass(entity);

        // Needs to be recalculated after we adjust the burn
        let end_time = self.path_component_mut(entity).last_segment_at_time(time).end_time();

        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, end_time);

        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.predict(entity, 1.0e10, SEGMENTS_TO_PREDICT);
    }

    pub fn can_create_burn(&mut self, entity: Entity) -> bool {
        let slots = self.vessel_component(entity).slots();
        slots.engine().is_some() && !slots.fuel_tanks().is_empty()
    }
}