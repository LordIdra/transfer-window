use nalgebra_glm::{vec2, DMat2, DVec2};
use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use self::burn_point::BurnPoint;

pub mod burn_point;

const MIN_DURATION: f64 = 0.1;
const BURN_TIME_STEP: f64 = 0.1;
const BURN_ACCELERATION_MAGNITUDE: f64 = 10.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct Burn {
    entity: Entity,
    parent: Entity,
    tangent: DVec2,
    delta_v: DVec2, // (tangent, normal)
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
    #[allow(clippy::too_many_arguments)]
    pub fn new(entity: Entity, parent: Entity, parent_mass: f64, tangent: DVec2, delta_v: DVec2, start_time: f64, start_position: DVec2, start_velocity: DVec2) -> Self {
        let start_point = BurnPoint::new(parent_mass, start_time, start_position, start_velocity);
        let mut burn = Self { 
            entity,
            parent,
            tangent,
            delta_v,
            current_point: start_point.clone(),
            points: vec![],
        };
        burn.recompute_burn_points(&start_point);
        burn
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn get_start_point(&self) -> &BurnPoint {
        self.points.first().unwrap()
    }

    pub fn get_current_point(&self) -> &BurnPoint {
        &self.current_point
    }
    
    #[allow(clippy::missing_panics_doc)]
    pub fn get_end_point(&self) -> &BurnPoint {
        self.points.last().unwrap()
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_total_dv(&self) -> f64 {
        self.delta_v.magnitude()
    }

    pub fn get_remaining_time(&self) -> f64 {
        self.get_end_point().get_time() - self.get_start_point().get_time()
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.get_start_point().get_time() && time < self.get_end_point().get_time()
    }

    pub fn get_tangent_direction(&self) -> DVec2 {
        self.tangent
    }

    pub fn get_duration(&self) -> f64 {
        f64::max(MIN_DURATION, self.get_total_dv() / BURN_ACCELERATION_MAGNITUDE)
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_point_at_time(&self, time: f64) -> BurnPoint {
        let time_after_start = time - self.get_start_point().get_time();
        let mut index = (time_after_start / BURN_TIME_STEP) as usize;
        index = index.saturating_sub(1);
        if let Some(closest_previous_point) = self.points.get(index) {
            let delta_time = time_after_start % BURN_TIME_STEP;
            closest_previous_point.next(delta_time, self.get_absolute_acceleration())
        } else {
            self.get_end_point().clone()
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.get_time() >= self.get_end_point().get_time()
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        time - self.get_end_point().get_time()
    }

    pub fn get_rotation_matrix(&self) -> DMat2 {
        DMat2::new(
            self.tangent.x, -self.tangent.y, 
            self.tangent.y, self.tangent.x)
    }

    fn get_absolute_delta_v(&self) -> DVec2 {
        self.get_rotation_matrix() * self.delta_v
    }

    fn get_absolute_acceleration(&self) -> DVec2 {
        let dv = self.get_absolute_delta_v();
        if dv.magnitude() == 0.0 {
            vec2(0.0, 0.0)
        } else {
            dv.normalize() * BURN_ACCELERATION_MAGNITUDE
        }
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.delta_v += adjustment;
        let start_point = self.get_start_point().clone();
        self.recompute_burn_points(&start_point);
    }

    fn recompute_burn_points(&mut self, start_point: &BurnPoint) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recompute burn points");
        let end_time = start_point.get_time() + self.get_duration();
        let mut points = vec![start_point.clone()];
        while points.last().unwrap().get_time() + BURN_TIME_STEP < end_time {
            points.push(points.last().unwrap().next(BURN_TIME_STEP, self.get_absolute_acceleration()));
        }
        let undershot_time = end_time - points.last().unwrap().get_time();
        points.push(points.last().unwrap().next(undershot_time, self.get_absolute_acceleration()));
        self.points = points;
    }

    pub fn reset(&mut self) {
        self.current_point = self.get_start_point().clone();
    }

    pub fn next(&mut self, delta_time: f64) {
        self.current_point = self.get_point_at_time(self.current_point.get_time() + delta_time);
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::trajectory_component::{brute_force_tester::BruteForceTester, burn::{Burn, BURN_ACCELERATION_MAGNITUDE, BURN_TIME_STEP}}, storage::entity_allocator::Entity};

    #[test]
    pub fn test() {
        let duration = 5.0 * 60.0;
        let entity = Entity::mock();
        let parent = Entity::mock();
        let parent_mass = 5.972e24; // earth's mass
        let tangent = vec2(1.0, 0.0);
        let acceleration = vec2(BURN_ACCELERATION_MAGNITUDE, 0.0);
        let delta_v = acceleration * duration;
        let start_time = 0.0;
        let start_position = vec2(2.00e6, 0.0);
        let start_velocity = vec2(0.0, 1.0e3);
        let mut burn = Burn::new(entity, parent, parent_mass, tangent, delta_v, start_time, start_position, start_velocity);
        
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        tester.update(duration);
        dbg!(tester.get_position(), burn.get_end_point().get_position());
        assert!((tester.get_time() - burn.get_end_point().get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_end_point().get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_end_point().get_velocity()).magnitude() < 1.0);
        
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        tester.update(0.5 * duration);
        dbg!(tester.get_position(), burn.get_point_at_time(0.5 * duration).get_position());
        assert!((tester.get_time() - burn.get_point_at_time(0.5 * duration).get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_point_at_time(0.5 * duration).get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_point_at_time(0.5 * duration).get_velocity()).magnitude() < 1.0);

        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        tester.update(0.5 * duration);
        burn.next(0.5 * duration);
        assert!((tester.get_time() - burn.get_current_point().get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_current_point().get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_current_point().get_velocity()).magnitude() < 1.0);

        let tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        burn.reset();
        assert!((tester.get_time() - burn.get_current_point().get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_current_point().get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_current_point().get_velocity()).magnitude() < 1.0);
    }
}