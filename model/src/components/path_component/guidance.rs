use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};
use transfer_window_common::numerical_methods::itp::itp;

use crate::{components::vessel_component::Faction, storage::entity_allocator::Entity, Model};

use self::guidance_point::GuidancePoint;

use super::burn::rocket_equation_function::RocketEquationFunction;

const MAX_INTERCEPT_DISTANCE: f64 = 50.0;
const MAX_GUIDANCE_TIME: f64 = 60.0 * 60.0;
const GUIDANCE_TIME_STEP: f64 = 0.5;
const LINE_OF_SIGHT_RATE_DELTA: f64 = 0.1;
const PROPORTIONALITY_CONSTANT: f64 = 3.0;
const DISTANCE_DERIVATIVE_DELTA: f64 = 0.001;

pub mod guidance_point;

pub fn will_intercept(intercept_distance: f64) -> bool {
    intercept_distance < MAX_INTERCEPT_DISTANCE
}

fn proportional_guidance_acceleration(absolute_position: DVec2, target_absolute_position: DVec2, absolute_velocity: DVec2, target_absolute_velocity: DVec2) -> DVec2 {
    let displacement = absolute_position - target_absolute_position;
    let closing_speed = -(absolute_velocity - target_absolute_velocity).dot(&displacement) / displacement.magnitude();

    let future_displacement = ((absolute_position + absolute_velocity * LINE_OF_SIGHT_RATE_DELTA) - (target_absolute_position + target_absolute_velocity * LINE_OF_SIGHT_RATE_DELTA)) / LINE_OF_SIGHT_RATE_DELTA;
    let line_of_sight_rate = (f64::atan2(displacement.y, displacement.x) - f64::atan2(future_displacement.y, future_displacement.x)) / LINE_OF_SIGHT_RATE_DELTA;

    let acceleration_unit = vec2(-displacement.y, displacement.x).normalize();
    acceleration_unit * PROPORTIONALITY_CONSTANT * closing_speed * line_of_sight_rate
}

fn compute_guidance_points(model: &Model, parent: Entity, target: Entity, faction: Faction, start_rocket_equation_function: &RocketEquationFunction, start_point: &GuidancePoint) -> (bool, Vec<GuidancePoint>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute guidance points");
    let mut points = vec![start_point.clone()];

    let mut dv = 0.0;
    
    loop {
        let last = points.last().unwrap();
        let time = last.time();

        let Some(rocket_equation_function) = start_rocket_equation_function.step_by_dv(dv) else {
            // We've run out of fuel
            return (false, points);
        };

        // Check if we are on an intercept trajectory
        let distance_at_delta_time = |delta_time: f64| {
            // Mass is technically not correct, but the difference is almost certainly negligible, and it's not really important anyway
            let point = last.next(delta_time, rocket_equation_function.mass());
            (model.absolute_position_at_time(parent, point.time(), Some(faction)) + point.position() - model.absolute_position_at_time(target, point.time(), Some(faction))).magnitude()
        };
        let distance_prime_at_delta_time = |delta_time: f64| {
            (distance_at_delta_time(delta_time + DISTANCE_DERIVATIVE_DELTA) - distance_at_delta_time(delta_time)) / DISTANCE_DERIVATIVE_DELTA
        };
        if distance_prime_at_delta_time(0.0).is_sign_negative() && distance_prime_at_delta_time(GUIDANCE_TIME_STEP).is_sign_positive() {
            // Distance derivative sign flips, so we have a minimum distance within GUIDANCE_TIME_STEP
            let intercept_delta_time = itp(&distance_prime_at_delta_time, 0.0, GUIDANCE_TIME_STEP);
            let intercept_distance = distance_at_delta_time(intercept_delta_time);
            if will_intercept(intercept_distance) {
                // We have an intercept
                points.push(last.next(intercept_delta_time, rocket_equation_function.mass()));
                return (true, points);
            }
        }

        if time - start_point.time() > MAX_GUIDANCE_TIME {
            return (false, points);
        }

        let absolute_position = model.absolute_position_at_time(parent, time, Some(faction)) + last.position();
        let absolute_velocity = model.absolute_velocity_at_time(parent, time, Some(faction)) + last.velocity();
        let target_absolute_position = model.absolute_position_at_time(target, time, Some(faction));
        let target_absolute_velocity = model.absolute_velocity_at_time(target, time, Some(faction));
        let mass = rocket_equation_function.mass();

        let requested_acceleration = proportional_guidance_acceleration(absolute_position, target_absolute_position, absolute_velocity, target_absolute_velocity);

        // Make sure guidance acceleration does not exceed max acceleration
        let actual_acceleration = if requested_acceleration.magnitude() < rocket_equation_function.acceleration() {
            requested_acceleration
        } else {
            requested_acceleration.normalize() * rocket_equation_function.acceleration()
        };

        dv += actual_acceleration.magnitude() * GUIDANCE_TIME_STEP;
        points.push(last.next_with_new_acceleration_and_dv(GUIDANCE_TIME_STEP, mass, actual_acceleration));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guidance {
    parent: Entity,
    target: Entity,
    faction: Faction,
    rocket_equation_function: RocketEquationFunction,
    current_point: GuidancePoint,
    points: Vec<GuidancePoint>,
    will_intercept: bool,
}

impl Guidance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(model: &Model, parent: Entity, target: Entity, faction: Faction, parent_mass: f64, start_time: f64, rocket_equation_function: &RocketEquationFunction, start_position: DVec2, start_velocity: DVec2) -> Self {
        let start_point = GuidancePoint::new(parent_mass, rocket_equation_function.mass(), start_time, start_position, start_velocity, vec2(0.0, 0.0), 0.0);
        let (will_intercept, points) = compute_guidance_points(model, parent, target, faction, rocket_equation_function, &start_point);
        Self { 
            parent,
            target,
            faction,
            rocket_equation_function: rocket_equation_function.clone(),
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
            closest_previous_point.next(undershot_time, closest_previous_point.mass())
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

    pub fn rocket_equation_function(&self) -> RocketEquationFunction {
        self.rocket_equation_function.clone()
    }

    pub fn dv_at_time(&self, time: f64) -> f64 {
        self.point_at_time(time).dv()
    }

    /// `time` is absolute
    pub fn rocket_equation_function_at_time(&self, time: f64) -> RocketEquationFunction {
        // Slightly annoying workaround to make sure that if the guidance expends all our DV, there won't be a panic
        let dv = self.dv_at_time(time);
        match self.rocket_equation_function.step_by_dv(dv) {
            Some(rocket_equation_function) => rocket_equation_function,
            None => self.rocket_equation_function.end(),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn final_rocket_equation_function(&self) -> RocketEquationFunction {
        self.rocket_equation_function_at_time(self.end_point().time())
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
