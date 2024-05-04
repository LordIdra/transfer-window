use nalgebra_glm::{vec2, DMat2, DVec2};
use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::{burn_point::BurnPoint, rocket_equation_function::RocketEquationFunction};

pub mod burn_point;
pub mod rocket_equation_function;

const MIN_DURATION: f64 = 0.1;
const BURN_TIME_STEP: f64 = 0.1;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Burn {
    entity: Entity,
    parent: Entity,
    rocket_equation_function: RocketEquationFunction,
    tangent: DVec2,
    delta_v: DVec2, // (tangent, normal)
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
    #[allow(clippy::too_many_arguments)]
    pub fn new(entity: Entity, parent: Entity, parent_mass: f64, tangent: DVec2, delta_v: DVec2, start_time: f64, rocket_equation_function: RocketEquationFunction, start_position: DVec2, start_velocity: DVec2) -> Self {
        let start_point = BurnPoint::new(parent_mass, rocket_equation_function.mass(), start_time, start_position, start_velocity);
        let mut burn = Self { 
            entity,
            parent,
            rocket_equation_function,
            tangent,
            delta_v,
            current_point: start_point.clone(),
            points: vec![],
        };
        burn.recompute_burn_points(&start_point);
        burn
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
            let undershot_time = time - closest_previous_point.time();
            let mass = self.rocket_equation_function.step_by_time(time_since_start).unwrap().mass();
            closest_previous_point.next(undershot_time, mass, self.absolute_acceleration(time_since_start))
        } else {
            self.end_point().clone()
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn total_dv(&self) -> f64 {
        self.delta_v.magnitude()
    }

    pub fn remaining_time(&self) -> f64 {
        self.end_point().time() - self.start_point().time()
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.start_point().time() && time < self.end_point().time()
    }

    pub fn tangent_direction(&self) -> DVec2 {
        self.tangent
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn duration(&self) -> f64 {
        let final_rocket_equation_function = self.rocket_equation_function.step_by_dv(self.total_dv()).unwrap();
        f64::max(MIN_DURATION, final_rocket_equation_function.burn_time() - self.rocket_equation_function.burn_time())
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

    #[allow(clippy::missing_panics_doc)]
    pub fn rocket_equation_function_at_end_of_burn(&self) -> RocketEquationFunction {
        self.rocket_equation_function.step_by_time(self.duration()).unwrap()
    }

    pub fn overshot_time(&self, time: f64) -> f64 {
        time - self.end_point().time()
    }

    pub fn rotation_matrix(&self) -> DMat2 {
        DMat2::new(
            self.tangent.x, -self.tangent.y, 
            self.tangent.y, self.tangent.x)
    }

    fn absolute_delta_v(&self) -> DVec2 {
        self.rotation_matrix() * self.delta_v
    }

    fn absolute_acceleration(&self, time: f64) -> DVec2 {
        let dv = self.absolute_delta_v();
        if dv.magnitude() == 0.0 {
            vec2(0.0, 0.0)
        } else {
            dv.normalize() * self.rocket_equation_function.step_by_time(time).unwrap().acceleration()
        }
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.delta_v += adjustment;
        let start_point = self.start_point().clone();
        self.recompute_burn_points(&start_point);
    }

    fn recompute_burn_points(&mut self, start_point: &BurnPoint) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recompute burn points");
        self.points.clear();
        let end_time = start_point.time() + self.duration();
        self.points.push(start_point.clone());
        while self.end_point().time() + BURN_TIME_STEP < end_time {
            let last = self.points.last().unwrap();
            let time_since_start = self.time_since_start(last.time());
            let mass = self.rocket_equation_function.step_by_time(time_since_start).unwrap().mass();
            self.points.push(last.next(BURN_TIME_STEP, mass, self.absolute_acceleration(time_since_start)));
        }
        let undershot_time = end_time - self.end_point().time();
        let last = self.points.last().unwrap();
        let time_since_start = self.time_since_start(last.time());
        let mass = self.rocket_equation_function.step_by_time(time_since_start).unwrap().mass();
        self.points.push(last.next(undershot_time, mass, self.absolute_acceleration(time_since_start)));
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point().clone();
    }

    pub fn next(&mut self, delta_time: f64) {
        self.current_point = self.point_at_time(self.current_point.time() + delta_time);
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::path_component::{brute_force_tester::BruteForceTester, burn::{rocket_equation_function::RocketEquationFunction, Burn, BURN_TIME_STEP}}, storage::entity_allocator::Entity};

    #[test]
    pub fn test() {
        let duration = 100.0;
        let entity = Entity::mock();
        let parent = Entity::mock();
        let parent_mass = 5.972e24; // earth's mass
        let tangent = vec2(1.0, 0.0);
        let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 1.0, 0.0);
        let rocket_equation_function_clone = rocket_equation_function.clone();
        let acceleration_from_time = move |time: f64| rocket_equation_function_clone.step_by_time(time).unwrap().acceleration() * tangent;
        let delta_v = rocket_equation_function.end().used_dv() * tangent;
        let start_time = 0.0;
        let start_position = vec2(2.00e6, 0.0);
        let start_velocity = vec2(0.0, 1.0e3);
        let mut burn = Burn::new(entity, parent, parent_mass, tangent, delta_v, start_time, rocket_equation_function, start_position, start_velocity);
        
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(duration);
        assert!((tester.time() - burn.end_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.end_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.end_point().velocity()).magnitude() < 1.0);
        
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(0.5 * duration);
        assert!((tester.time() - burn.point_at_time(0.5 * duration).time()).abs() < 1.0);
        assert!((tester.position() - burn.point_at_time(0.5 * duration).position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.point_at_time(0.5 * duration).velocity()).magnitude() < 1.0);

        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time.clone(), BURN_TIME_STEP);
        tester.update(0.5 * duration);
        burn.next(0.5 * duration);
        assert!((tester.time() - burn.current_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.current_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.current_point().velocity()).magnitude() < 1.0);

        let tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time, BURN_TIME_STEP);
        burn.reset();
        assert!((tester.time() - burn.current_point().time()).abs() < 1.0);
        assert!((tester.position() - burn.current_point().position()).magnitude() < 1.0);
        assert!((tester.velocity() - burn.current_point().velocity()).magnitude() < 1.0);
    }
}