use nalgebra_glm::{vec2, DVec2};

use crate::{components::{path_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, orbit::Orbit, segment::Segment}, vessel_component::system_slot::System}, storage::entity_allocator::Entity, Model};

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
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }

    pub fn create_burn(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);

        let parent = path_component.last_segment().parent();
        let tangent = path_component.last_segment().end_velocity().normalize();
        let start_position = path_component.last_segment().end_position();
        let start_velocity = path_component.last_segment().end_velocity();
        let rocket_equation_function = self.rocket_equation_function_at_end_of_trajectory(entity);
        let parent_mass = self.mass(parent);
        let burn = Burn::new(entity, parent, parent_mass, tangent, vec2(0.0, 0.0), time, rocket_equation_function, start_position, start_velocity);

        let end_point = burn.end_point();
        let orbit = Orbit::new(parent, end_point.mass(), parent_mass, end_point.position(), end_point.velocity(), end_point.time());

        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if there is no burn at the specified time
    pub fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let path_component = self.path_component_mut(entity);
        // The reason we do start time instead of end time is because a burn could have a duration of 0
        // This would mean we have eg (orbit 0.0 <-> 100) then (burn 100 <-> 100) then (orbit 100 <-> 160)
        // In this case we'd have to be careful to remove only the orbit starting at 100, which is difficult
        let mut burn = path_component.future_segment_starting_at_time(time)
            .unwrap_or_else(|| panic!("Burn not found at time {time}"))
            .as_burn()
            .unwrap_or_else(|| panic!("Burn not found at time {time}"))
            .clone();
        path_component.remove_segments_after(burn.start_point().time());
        burn.adjust(amount);
        let mass = burn.end_point().mass();
        path_component.add_segment(Segment::Burn(burn));

        let end_segment = path_component.last_segment();
        let parent = end_segment.parent();
        let position = end_segment.end_position();
        let velocity = end_segment.end_velocity();
        let parent_mass = self.mass(parent);

        // Needs to be recalculated after we adjust the burn
        let end_time = self.path_component_mut(entity)
            .future_segment_starting_at_time(time)
            .unwrap_or_else(|| panic!("Burn not found at time {time}"))
            .end_time();

        let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, end_time);

        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub fn can_create_burn(&self, entity: Entity) -> bool {
        let slots = self.vessel_component(entity).slots();
        slots.engine().is_some() && !slots.fuel_tanks().is_empty()
    }

    /// # Panics
    /// Panics if the entity does not have an orbit at the given time
    pub fn orbit_at_time(&self, entity: Entity, time: f64) -> &Orbit {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Orbit at time");
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            if let Some(orbit) = orbitable_component.orbit() {
                return orbit;
            }
        }

        if let Some(path_component) = self.try_path_component(entity) {
            if let Segment::Orbit(orbit) = path_component.future_segment_at_time(time) {
                return orbit;
            }
        }

        panic!("There is no orbit at the requested time")
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
}