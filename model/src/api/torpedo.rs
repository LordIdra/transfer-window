use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};
use transfer_window_common::numerical_methods::laguerre::laguerre_to_find_stationary_point_option_mut;

use crate::{components::{name_component::NameComponent, path_component::{orbit::Orbit, PathComponent}, vessel_component::{system_slot::SlotLocation, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

const EXPEL_VELOCITY: f64 = 10.0;
const TIME_BEFORE_BURN_START: f64 = 5.0;

impl Model {
    fn torpedo_guidance(&mut self, entity: Entity, target: Entity, max_dv: f64) -> Option<DVec2> {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Torpedo guidance");
        let burn_time = self.time + TIME_BEFORE_BURN_START;
        let original_dv = vec2(0.0, max_dv);
        let time = self.time;
        let compute_dv = |angle: f64| vec2(
            original_dv.x * angle.cos() - original_dv.y * angle.sin(), 
            original_dv.x * angle.sin() + original_dv.y * angle.cos());
        let mut closest_approach_from_angle = |angle: f64| {
            self.create_burn(entity, burn_time);
            self.adjust_burn(entity, burn_time, compute_dv(angle));
            let time = self.find_next_closest_approach(entity, target, burn_time)?;
            let distance = (self.position_at_time(entity, time) - self.position_at_time(target, time)).magnitude();
            self.path_component_mut(entity).remove_segments_after(burn_time);
            Some((angle, time, distance))
        };
        
        // Stage 1: brute force 80 points and find their closest approach times
        let mut times = vec![];
        for i in 0..60 {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Test one intial angle");
            let angle = (i as f64 / 100.0) * 2.0 * PI;
            if let Some(approach_time) = closest_approach_from_angle(angle) {
                times.push(approach_time);
            };
        }

        const MAX_TIME: f64 = 60.0 * 60.0 * 24.0 * 7.0;

        // Stage 2: Filter unreasonable points
        let mut results: Vec<&(f64, f64, f64)> = times.iter()
            .filter(|x| (x.1 - time) < MAX_TIME)
            .collect();
        results.sort_by(|a, b| a.2.total_cmp(&b.2));
        let angle = results.first()?.0;
        
        // Stage 2: Laguerre
        let mut to_minimise = |angle: f64| closest_approach_from_angle(angle).map(|x| x.2);
        let result = laguerre_to_find_stationary_point_option_mut(&mut to_minimise, angle, 1.0e-4, 1.0e-3, 100)?;
        Some(compute_dv(result))

        // Stage 2: Find the derivative of distance and keep going in that direction until the sign flips
        // const DERIVATIVE_DELTA: f64 = 0.01;
        // let mut derivative = |angle: f64| {
        //     let a = closest_approach_from_angle(angle + DERIVATIVE_DELTA)?.2;
        //     let b = closest_approach_from_angle(angle)?.2;
        //     Some((a - b) / DERIVATIVE_DELTA)
        // };
        // let derivative_direction = derivative(angle)?.signum();
        // let mut increment = 0.01 * derivative_direction;
        // let mut angle = angle;
        // const MULTIPLIER: f64 = 1.8;
        // const MAX_INCREMENT: f64 = 15.0;
        // loop {
        //     if increment >= MAX_INCREMENT {
        //         return None;
        //     }
        //     increment *= MULTIPLIER;
        //     angle += increment;
        //     if derivative(angle)?.signum() != derivative_direction {
        //         // Direction has flipped; we have our bounds
        //         let from = angle - increment;
        //         let to = angle;
                
        //     }
        // }
    }

    pub fn spawn_torpedo(&mut self, fire_from: Entity, slot_location: SlotLocation, target: Entity) {
        let parent = self.path_component(fire_from).current_segment().parent();
        let mut vessel_component = VesselComponent::new(VesselClass::Torpedo);
        // Subtract 0.1 because otherwise floating point errors might mean the burn is impossible
        let max_dv = vessel_component.max_dv().unwrap() - 1.0e-5;
        vessel_component.set_target(Some(target));
        let velocity = self.velocity(fire_from);
        let extra_velocity = vec2(-velocity.y, velocity.x).normalize() * EXPEL_VELOCITY;
        let orbit = Orbit::new(parent, vessel_component.mass(), self.mass(parent), self.position(fire_from), velocity + extra_velocity, self.time);
        let entity = self.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("Torpedo".to_string()))
            .with_vessel_component(vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)));
        self.recompute_trajectory(entity);
        let burn_time = self.time + TIME_BEFORE_BURN_START;
        if let Some(dv) = self.torpedo_guidance(entity, target, max_dv) {
            self.create_burn(entity, burn_time);
            self.adjust_burn(entity, burn_time, dv);
        }
    }
}