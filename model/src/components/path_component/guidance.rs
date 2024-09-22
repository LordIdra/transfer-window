use log::error;
use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};
use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::vessel_component::{engine::Engine, faction::Faction}, storage::entity_allocator::Entity, Model};

use self::guidance_point::GuidancePoint;

use super::rocket_equation_function::RocketEquationFunction;

const MAX_INTERCEPT_DISTANCE: f64 = 50.0;
const MAX_GUIDANCE_TIME: f64 = 60.0 * 60.0;
const GUIDANCE_TIME_STEP: f64 = 0.5;
const LINE_OF_SIGHT_RATE_DELTA: f64 = 0.1;
const PROPORTIONALITY_CONSTANT: f64 = 3.0;
const DISTANCE_DERIVATIVE_DELTA: f64 = 0.001;

pub mod builder;
pub mod guidance_point;

pub fn will_intercept(intercept_distance: f64) -> bool {
    intercept_distance < MAX_INTERCEPT_DISTANCE
}

fn proportional_guidance_acceleration(absolute_position: DVec2, target_absolute_position: DVec2, absolute_velocity: DVec2, target_absolute_velocity: DVec2) -> DVec2 {
    let displacement = absolute_position - target_absolute_position;
    let closing_speed = -(absolute_velocity - target_absolute_velocity).dot(&displacement) / displacement.magnitude();

    let future_position = absolute_position + absolute_velocity * LINE_OF_SIGHT_RATE_DELTA;
    let future_target_position = target_absolute_position + target_absolute_velocity * LINE_OF_SIGHT_RATE_DELTA;
    let future_displacement = (future_position - future_target_position) / LINE_OF_SIGHT_RATE_DELTA;
    let line_of_sight_rate = (f64::atan2(displacement.y, displacement.x) - f64::atan2(future_displacement.y, future_displacement.x)) / LINE_OF_SIGHT_RATE_DELTA;

    let acceleration_unit = vec2(-displacement.y, displacement.x).normalize();
    acceleration_unit * PROPORTIONALITY_CONSTANT * closing_speed * line_of_sight_rate
}

fn compute_guidance_points(
    model: &Model, 
    parent: Entity, 
    target: Entity, 
    faction: Faction, 
    engine: &Engine,
    start_point: &GuidancePoint,
) -> (bool, Vec<GuidancePoint>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute guidance points");
    let mut points = vec![start_point.clone()];

    loop {
        let last = points.last().unwrap();
        let time = last.time();

        // Check we're not out of fuel
        if last.fuel_kg() < engine.fuel_kg_per_second() * GUIDANCE_TIME_STEP {
            return (false, points);
        };

        // Check we're not over max guidance time
        if time - start_point.time() > MAX_GUIDANCE_TIME {
            return (false, points);
        }

        // Check if we are on an intercept trajectory
        let distance_at_delta_time = |delta_time: f64| {
            let point = last.next(delta_time, engine);
            let parent_absolute_position = model.absolute_position_at_time(parent, point.time(), Some(faction));
            let target_absolute_position = model.absolute_position_at_time(target, point.time(), Some(faction));
            (parent_absolute_position + point.position() - target_absolute_position).magnitude()
        };
        let distance_prime_at_delta_time = |delta_time: f64| {
            (distance_at_delta_time(delta_time + DISTANCE_DERIVATIVE_DELTA) - distance_at_delta_time(delta_time)) / DISTANCE_DERIVATIVE_DELTA
        };
        if distance_prime_at_delta_time(0.0).is_sign_negative() && distance_prime_at_delta_time(GUIDANCE_TIME_STEP).is_sign_positive() {
            // Distance derivative sign flips, so we have a minimum distance within GUIDANCE_TIME_STEP
            match itp(&distance_prime_at_delta_time, 0.0, GUIDANCE_TIME_STEP) {
                Err(err) => error!("Error while checking for intercept: {}", err),
                Ok(intercept_delta_time) => {
                    let intercept_distance = distance_at_delta_time(intercept_delta_time);
                    if will_intercept(intercept_distance) {
                        // We have an intercept
                        points.push(last.next(intercept_delta_time, engine));
                        return (true, points);
                    }
                },
            }
        }

        // Calculate acceleration
        let absolute_position = model.absolute_position_at_time(parent, time, Some(faction)) + last.position();
        let absolute_velocity = model.absolute_velocity_at_time(parent, time, Some(faction)) + last.velocity();
        let target_absolute_position = model.absolute_position_at_time(target, time, Some(faction));
        let target_absolute_velocity = model.absolute_velocity_at_time(target, time, Some(faction));
        let requested_acceleration = proportional_guidance_acceleration(absolute_position, target_absolute_position, absolute_velocity, target_absolute_velocity);

        // Make sure guidance acceleration does not exceed max acceleration
        let max_acceleration = engine.thrust_newtons() / last.mass();
        let actual_acceleration = if requested_acceleration.magnitude() < max_acceleration {
            requested_acceleration
        } else {
            requested_acceleration.normalize() * max_acceleration
        };

        // Finally, set acceleration and construct the next point
        let last = points.last_mut().unwrap();
        last.set_acceleration(actual_acceleration);
        let next = last.next(GUIDANCE_TIME_STEP, engine);
        points.push(next);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guidance {
    parent: Entity,
    target: Entity,
    faction: Faction,
    engine: Engine,
    current_point: GuidancePoint,
    points: Vec<GuidancePoint>,
    will_intercept: bool,
}

impl Guidance {
    #[allow(clippy::too_many_arguments)]
    pub(self) fn new(
        model: &Model, 
        parent: Entity, 
        parent_mass: f64, 
        target: Entity, 
        faction: Faction, 
        engine: &Engine, 
        mass: f64,
        fuel_kg: f64,
        start_time: f64, 
        start_rotation: f64, 
        start_position: DVec2, 
        start_velocity: DVec2
    ) -> Self {
        let mass_without_fuel = mass - fuel_kg;
        let start_point = GuidancePoint::new(parent_mass, mass_without_fuel, fuel_kg, start_time, start_rotation, start_position, start_velocity, vec2(0.0, 0.0));
        let (will_intercept, points) = compute_guidance_points(model, parent, target, faction, engine, &start_point);
        Self { 
            parent,
            target,
            faction,
            engine: engine.clone(),
            current_point: start_point.clone(),
            points,
            will_intercept,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn start_point(&self) -> &GuidancePoint {
        self.points.first().unwrap()
    }

    pub fn current_point(&self) -> &GuidancePoint {
        &self.current_point
    }
    
    #[allow(clippy::missing_panics_doc)]
    pub fn end_point(&self) -> &GuidancePoint {
        self.points.last().unwrap()
    }

    /// `time` is absolute
    #[allow(clippy::missing_panics_doc)]
    pub fn point_at_time(&self, time: f64) -> GuidancePoint {
        let time_since_start = self.time_since_start(time);
        let index = (time_since_start / GUIDANCE_TIME_STEP) as usize;
        if let Some(closest_previous_point) = self.points.get(index) {
            let undershot_time = time - closest_previous_point.time();
            closest_previous_point.next(undershot_time, &self.engine)
        } else {
            self.end_point().clone()
        }
    }

    pub fn remaining_time(&self) -> f64 {
        self.end_point().time() - self.current_point().time()
    }

    pub fn is_time_within_guidance(&self, time: f64) -> bool {
        time > self.start_point().time() && time < self.end_point().time()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn duration(&self) -> f64 {
        self.end_point().time() - self.start_point().time()
    }

    pub fn parent(&self) -> Entity {
        self.parent
    }

    pub fn target(&self) -> Entity {
        self.target
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.time() >= self.end_point().time()
    }

    pub fn time_since_start(&self, absolute_time: f64) -> f64 {
        absolute_time - self.start_point().time()
    }

    fn start_rocket_equation_function(&self) -> RocketEquationFunction {
        RocketEquationFunction::new(self.start_point().dry_mass(), 
            self.start_point().fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    fn end_rocket_equation_function(&self) -> RocketEquationFunction {
        RocketEquationFunction::new(self.end_point().dry_mass(), 
            self.end_point().fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    fn rocket_equation_function_at_time(&self, time: f64) -> RocketEquationFunction {
        RocketEquationFunction::new(self.point_at_time(time).dry_mass(), 
            self.point_at_time(time).fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    pub fn start_remaining_dv(&self) -> f64 {
        self.start_rocket_equation_function().remaining_dv()
    }

    pub fn start_fuel_kg(&self) -> f64 {
        self.start_rocket_equation_function().remaining_fuel_kg()
    }

    pub fn end_remaining_dv(&self) -> f64 {
        self.end_rocket_equation_function().remaining_dv()
    }

    pub fn end_fuel_kg(&self) -> f64 {
        self.end_rocket_equation_function().remaining_fuel_kg()
    }

    pub fn remaining_dv_at_time(&self, time: f64) -> f64 {
        self.rocket_equation_function_at_time(time).remaining_dv()
    }

    pub fn fuel_kg_at_time(&self, time: f64) -> f64 {
        self.rocket_equation_function_at_time(time).remaining_fuel_kg()
    }

    pub fn will_intercept(&self) -> bool {
        self.will_intercept
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point().clone();
    }

    pub fn next(&mut self, time: f64) {
        let delta_time = time - self.current_point.time();
        self.current_point = self.point_at_time(self.current_point.time() + delta_time);
    }
}
