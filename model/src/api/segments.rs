use nalgebra_glm::{vec2, DVec2};

use crate::{components::{path_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, guidance::Guidance, orbit::Orbit, segment::Segment}, vessel_component::timeline::{intercept::InterceptEvent, TimelineEvent}}, storage::entity_allocator::Entity, Model};

const MIN_DV_TO_ADJUST_BURN: f64 = 1.0e-2;

impl Model {
    pub(crate) fn rocket_equation_function_at_end_of_trajectory(&self, entity: Entity) -> RocketEquationFunction {
        if let Some(rocket_equation_function) = self.path_component(entity).final_rocket_equation_function() {
            return rocket_equation_function;
        }

        RocketEquationFunction::from_vessel_component(self.vessel_component(entity))
    }

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

    fn add_guidance(&mut self, entity: Entity, parent: Entity, guidance: Guidance) {
        let end_point = guidance.end_point().clone();
        let will_intercept = guidance.will_intercept();
        let target = guidance.target();
        self.path_component_mut(entity).add_segment(Segment::Guidance(guidance));

        if will_intercept {
            let intercept_time = end_point.time();
            let event = TimelineEvent::Intercept(InterceptEvent::new(self, entity, target, intercept_time));
            self.vessel_component_mut(entity)
                .timeline_mut()
                .add(event);
        }
        
        let parent_mass = self.mass(parent);
        let orbit = Orbit::new(parent, end_point.mass(), parent_mass, end_point.position(), end_point.velocity(), end_point.time());
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn create_guidance(&mut self, entity: Entity, time: f64) {
        assert!(self.vessel_component_mut(entity).class_mut().is_torpedo());

        let target = self.vessel_component(entity)
            .target()
            .expect("Cannot enable guidance on torpedo without a target");
        
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);

        let last_segment = path_component.final_segment();
        let parent = last_segment.parent();
        let start_position = last_segment.end_position();
        let start_velocity = last_segment.end_velocity();
        let parent_mass = self.mass(parent);
        let rocket_equation_function = self.rocket_equation_function_at_end_of_trajectory(entity);
        let guidance = Guidance::new(self, parent, target, parent_mass, time, &rocket_equation_function, start_position, start_velocity);
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn recalculate_current_guidance(&mut self, entity: Entity) {
        assert!(self.path_component(entity).future_segments().len() == 1);
        assert!(self.path_component(entity).current_segment().is_guidance());
        assert!(self.path_component(entity).current_segment().as_guidance().unwrap().will_intercept());

        let guidance = self.path_component(entity)
            .current_segment()
            .as_guidance()
            .unwrap();
        let parent = guidance.parent();
        let target = guidance.target();
        let start_position = guidance.current_point().position();
        let start_velocity = guidance.current_point().velocity();
        let parent_mass = self.mass(parent);
        let rocket_equation_function = guidance.rocket_equation_function_at_time(self.time);
        let guidance = Guidance::new(self, parent, target, parent_mass, self.time, &rocket_equation_function, start_position, start_velocity);
        self.path_component_mut(entity).clear_future_segments();
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn delete_guidance(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
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

    pub fn parent(&self, entity: Entity) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.orbit().map(|orbit| orbit.parent());
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return Some(path_component.current_segment().parent());
        }

        None
    }

    pub fn parent_at_time(&self, entity: Entity, time: f64) -> Option<Entity> {
        if let Some(orbitable_component) = self.try_orbitable_component(entity) {
            return orbitable_component.orbit().map(|orbit| orbit.parent());
        }

        if let Some(path_component) = self.try_path_component(entity) {
            return Some(path_component.future_segment_at_time(time).parent());
        }

        None
    }
}