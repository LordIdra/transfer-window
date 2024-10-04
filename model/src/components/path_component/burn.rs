use nalgebra_glm::{DMat2, DVec2};
use serde::{Deserialize, Serialize};

use crate::{components::vessel_component::engine::Engine, storage::entity_allocator::Entity};

use self::burn_point::BurnPoint;

use super::rocket_equation_function::RocketEquationFunction;

pub mod builder;
pub mod burn_point;

const BURN_TIME_STEP: f64 = 0.1;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Burn {
    parent: Entity,
    engine: Engine,
    rotation: f64,
    tangent: DVec2,
    dv: DVec2, // (tangent, normal)
    max_dv: f64,
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
    #[allow(clippy::too_many_arguments)]
    pub(self) fn new(
        parent: Entity, 
        parent_mass: f64, 
        mass: f64, 
        fuel_kg: f64, 
        engine: &Engine, 
        tangent: DVec2, 
        mut dv: DVec2, 
        time: f64, 
        position: DVec2, 
        velocity: DVec2
    ) -> Self {
        let mass_without_fuel = mass - fuel_kg;
        let max_dv = RocketEquationFunction::new(mass_without_fuel, fuel_kg, engine.fuel_kg_per_second(), engine.specific_impulse())
            .remaining_dv();
        if dv.magnitude() > max_dv {
            dv = dv.normalize() * max_dv;
        }

        let start_point = BurnPoint::new(parent_mass, mass_without_fuel, fuel_kg, time, position, velocity);
        let mut burn = Self { 
            parent,
            engine: engine.clone(),
            rotation: 0.0, // initialised later
            tangent,
            dv,
            max_dv,
            current_point: start_point.clone(),
            points: vec![],
        };
        burn.recompute_burn_points(&start_point);
        burn.rotation = f64::atan2(burn.absolute_delta_v().y, burn.absolute_delta_v().x);
        burn
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn start_point(&self) -> &BurnPoint {
        self.points.first().unwrap()
    }

    pub fn current_point(&self) -> &BurnPoint {
        &self.current_point
    }
    
    #[allow(clippy::missing_panics_doc)]
    pub fn end_point(&self) -> &BurnPoint {
        self.points.last().unwrap()
    }

    /// `time` is absolute
    #[allow(clippy::missing_panics_doc)]
    pub fn point_at_time(&self, time: f64) -> BurnPoint {
        let time_since_start = self.time_since_start(time);
        let index = (time_since_start / BURN_TIME_STEP) as usize;
        if let Some(closest_previous_point) = self.points.get(index) {
            let base_time = closest_previous_point.time();
            let undershot_time = time - base_time;
            closest_previous_point.next(undershot_time, &self.engine, self.absolute_delta_v())
        } else {
            self.end_point().clone()
        }
    }

    pub fn total_dv(&self) -> f64 {
        self.dv.magnitude()
    }

    pub fn delta_v(&self) -> DVec2 {
        self.dv
    }

    pub fn remaining_time(&self) -> f64 {
        self.end_point().time() - self.current_point().time()
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.start_point().time() && time < self.end_point().time()
    }

    pub fn tangent_direction(&self) -> DVec2 {
        self.tangent
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

    pub fn rotation_matrix(&self) -> DMat2 {
        DMat2::new(
            self.tangent.x, -self.tangent.y, 
            self.tangent.y, self.tangent.x)
    }

    fn absolute_delta_v(&self) -> DVec2 {
        self.rotation_matrix() * self.dv
    }

    fn start_rocket_equation(&self) -> RocketEquationFunction {
        RocketEquationFunction::new(
            self.start_point().mass_without_fuel(),
            self.start_point().fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    fn end_rocket_equation(&self) -> RocketEquationFunction {
        RocketEquationFunction::new(
            self.end_point().mass_without_fuel(),
            self.end_point().fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    fn rocket_equation_at_time(&self, time: f64) -> RocketEquationFunction {
        RocketEquationFunction::new(
            self.point_at_time(time).mass_without_fuel(),
            self.point_at_time(time).fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse())
    }

    pub fn start_remaining_dv(&self) -> f64 {
        self.start_rocket_equation().remaining_dv()
    }

    pub fn start_fuel_kg(&self) -> f64 {
        self.start_rocket_equation().fuel_kg()
    }

    pub fn end_remaining_dv(&self) -> f64 {
        self.end_rocket_equation().remaining_dv()
    }

    pub fn end_fuel_kg(&self) -> f64 {
        self.end_rocket_equation().fuel_kg()
    }

    pub fn remaining_dv_at_time(&self, time: f64) -> f64 {
        self.rocket_equation_at_time(time).remaining_dv()
    }

    pub fn fuel_kg_at_time(&self, time: f64) -> f64 {
        self.rocket_equation_at_time(time).fuel_kg()
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.dv += adjustment;
        if self.dv.magnitude() > self.max_dv {
            self.dv = self.dv.normalize() * (self.max_dv - 0.000_000_1);
        }

        let start_point = self.start_point().clone();
        self.rotation = f64::atan2(self.absolute_delta_v().y, self.absolute_delta_v().x);
        self.recompute_burn_points(&start_point);
    }

    fn recompute_burn_points(&mut self, start_point: &BurnPoint) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recompute burn points");
        let duration = RocketEquationFunction::new(
            start_point.mass_without_fuel(), 
            start_point.fuel_kg(), 
            self.engine.fuel_kg_per_second(), 
            self.engine.specific_impulse(), 
        ).time_to_step_dv(self.dv.magnitude()).unwrap();

        let end_time = start_point.time() + duration;

        self.points.clear();
        self.points.push(start_point.clone());
        
        while self.end_point().time() + BURN_TIME_STEP < end_time {
            let last = self.points.last().unwrap();
            self.points.push(last.next(BURN_TIME_STEP, &self.engine, self.absolute_delta_v()));
        }

        if duration != 0.0 {
            let undershot_dt = end_time - self.end_point().time();
            let last = self.points.last().unwrap();
            self.points.push(last.next(undershot_dt, &self.engine, self.absolute_delta_v()));
        }
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point().clone();
    }

    pub fn next(&mut self, time: f64) {
        self.current_point = self.point_at_time(time);
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::{path_component::{brute_force_tester::BruteForceTester, burn::{builder::BurnBuilder, RocketEquationFunction, BURN_TIME_STEP}}, vessel_component::engine::Engine}, storage::entity_allocator::Entity};

    #[test]
    pub fn test() {
        let parent_mass = 5.972e24; // earth's mass
        let duration = 100.0;
        let tangent = vec2(1.0, 0.0);
        let dry_mass = 100.0; 
        let fuel_mass = 100.0;
        let engine = Engine::new(1.0, 1.0);
        let rocket_equation_function = RocketEquationFunction::new(dry_mass, fuel_mass, engine.fuel_kg_per_second(), engine.specific_impulse());
        let rocket_equation_function_clone = rocket_equation_function.clone();
        let acceleration_from_time = move |time: f64| rocket_equation_function_clone.step_by_time(time).unwrap().acceleration() * tangent;
        let position = vec2(2.00e6, 0.0);
        let velocity = vec2(0.0, 1.0e3);

        let mut burn = BurnBuilder{
            parent: Entity::mock(),
            parent_mass,
            tangent,
            delta_v: rocket_equation_function.remaining_dv() * tangent,
            time: 0.0,
            mass: dry_mass + fuel_mass,
            fuel_kg: fuel_mass,
            engine,
            position,
            velocity
        }.build();
        
        let mut tester = BruteForceTester::new(parent_mass, position, velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(duration);
        assert!((tester.time() - burn.end_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.end_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.end_point().velocity()).magnitude() < 1.0);
        
        let mut tester = BruteForceTester::new(parent_mass, position, velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(0.5 * duration);
        assert!((tester.time() - burn.point_at_time(0.5 * duration).time()).abs() < 1.0);
        assert!((tester.position() - burn.point_at_time(0.5 * duration).position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.point_at_time(0.5 * duration).velocity()).magnitude() < 1.0);

        let mut tester = BruteForceTester::new(parent_mass, position, velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(0.5 * duration);
        burn.next(0.5 * duration);
        assert!((tester.time() - burn.current_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.current_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.current_point().velocity()).magnitude() < 1.0);

        let tester = BruteForceTester::new(parent_mass, position, velocity, acceleration_from_time, BURN_TIME_STEP);
        burn.reset();
        assert!((tester.time() - burn.current_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.current_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.current_point().velocity()).magnitude() < 1.0);
    }
}
