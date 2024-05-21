use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

use crate::{storage::entity_allocator::Entity, Model};

use self::guidance_point::GuidancePoint;

use super::burn::rocket_equation_function::RocketEquationFunction;

const INTERCEPT_DISTANCE: f64 = 100.0;
pub const MAX_GUIDANCE_TIME: f64 = 60.0 * 30.0;
const GUIDANCE_TIME_STEP: f64 = 0.05;
const LINE_OF_SIGHT_RATE_DELTA: f64 = 0.1;
const PROPORTIONALITY_CONSTANT: f64 = 3.0;

pub mod guidance_point;

fn proportional_guidance_acceleration(absolute_position: DVec2, target_absolute_position: DVec2, absolute_velocity: DVec2, target_absolute_velocity: DVec2) -> DVec2 {
    let displacement = absolute_position - target_absolute_position;
    let closing_speed = -(absolute_velocity - target_absolute_velocity).dot(&displacement) / displacement.magnitude();

    let future_displacement = ((absolute_position + absolute_velocity * LINE_OF_SIGHT_RATE_DELTA) - (target_absolute_position + target_absolute_velocity * LINE_OF_SIGHT_RATE_DELTA)) / LINE_OF_SIGHT_RATE_DELTA;
    let line_of_sight_rate = (f64::atan2(displacement.y, displacement.x) - f64::atan2(future_displacement.y, future_displacement.x)) / LINE_OF_SIGHT_RATE_DELTA;

    let acceleration_unit = vec2(-displacement.y, displacement.x).normalize();
    acceleration_unit * PROPORTIONALITY_CONSTANT * closing_speed * line_of_sight_rate
}

fn compute_guidance_points(model: &Model, parent: Entity, target: Entity, start_rocket_equation_function: &RocketEquationFunction, start_point: &GuidancePoint) -> Vec<GuidancePoint> {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Compute guidance points");
    let mut points = vec![start_point.clone()];

    let mut dv = 0.0;
    
    loop {
        let last = points.last().unwrap();
        let time = last.time() + GUIDANCE_TIME_STEP;
        let time_since_start = time - start_point.time();

        let absolute_position = model.absolute_position_at_time(parent, time) + last.position();
        let absolute_velocity = model.absolute_velocity_at_time(parent, time) + last.velocity();
        let target_absolute_position = model.absolute_position_at_time(target, time);
        let target_absolute_velocity = model.absolute_velocity_at_time(target, time);

        let distance = (absolute_position - target_absolute_position).magnitude();
        if distance < INTERCEPT_DISTANCE {
            // TODO make this account for intercept between 2 points lol
            break;
        }

        if time_since_start > MAX_GUIDANCE_TIME {
            break;
        }

        let Some(rocket_equation_function) = start_rocket_equation_function.step_by_dv(dv) else {
            break;
        };

        let mass = rocket_equation_function.mass();
        let guidance_acceleration = proportional_guidance_acceleration(absolute_position, target_absolute_position, absolute_velocity, target_absolute_velocity);
        dv += guidance_acceleration.magnitude() * GUIDANCE_TIME_STEP;
        points.push(last.next_with_new_acceleration_and_dv(GUIDANCE_TIME_STEP, mass, guidance_acceleration));
    }

    points
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guidance {
    parent: Entity,
    rocket_equation_function: RocketEquationFunction,
    current_point: GuidancePoint,
    points: Vec<GuidancePoint>,
}

impl Guidance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(model: &Model, parent: Entity, target: Entity, parent_mass: f64, start_time: f64, rocket_equation_function: &RocketEquationFunction, start_position: DVec2, start_velocity: DVec2) -> Self {
        let start_point = GuidancePoint::new(parent_mass, rocket_equation_function.mass(), start_time, start_position, start_velocity, vec2(0.0, 0.0), 0.0);
        let points = compute_guidance_points(model, parent, target, rocket_equation_function, &start_point);
        Self { 
            parent,
            rocket_equation_function: rocket_equation_function.clone(),
            current_point: start_point.clone(),
            points,
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
            let mass = self.rocket_equation_function()
                .step_by_dv(closest_previous_point.dv())
                .unwrap()
                .mass();
            closest_previous_point.next(undershot_time, mass)
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
        let dv = self.dv_at_time(time - self.start_point().time());
        match self.rocket_equation_function.step_by_dv(dv) {
            Some(rocket_equation_function) => rocket_equation_function,
            None => self.rocket_equation_function.end(),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rocket_equation_function_at_end_of_guidance(&self) -> RocketEquationFunction {
        self.rocket_equation_function_at_time(self.end_point().time())
    }

    pub fn overshot_time(&self, time: f64) -> f64 {
        time - self.end_point().time()
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point().clone();
    }

    pub fn next(&mut self, delta_time: f64) {
        self.current_point = self.point_at_time(self.current_point.time() + delta_time);
    }
}
