use nalgebra_glm::{DVec2, vec2};

use crate::{storage::entity_allocator::Entity, constants::{BURN_ACCELERATION_MAGNITUDE, BURN_TIME_STEP}};

use self::burn_point::BurnPoint;

mod burn_point;

pub struct Burn {
    entity: Entity,
    parent: Entity,
    tangent: DVec2,
    delta_v: DVec2, // (tangent, normal)
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
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
        burn.recompute_burn_points(start_point);
        burn
    }

    pub fn get_start_point(&self) -> &BurnPoint {
        self.points.first().unwrap()
    }

    pub fn get_current_point(&self) -> &BurnPoint {
        &self.current_point
    }
    
    pub fn get_end_point(&self) -> &BurnPoint {
        self.points.last().unwrap()
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_total_dv(&self) -> f64 {
        self.delta_v.magnitude()
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.get_start_point().get_time() && time < self.get_end_point().get_time()
    }

    pub fn get_tangent_direction(&self) -> DVec2 {
        self.tangent
    }

    pub fn get_duration(&self) -> f64 {
        self.get_total_dv() / BURN_ACCELERATION_MAGNITUDE
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_point_at_time(&self, time: f64) -> BurnPoint {
        let time_after_start = time - self.get_start_point().get_time();
        if let Some(closest_previous_point) = self.points.get((time_after_start / BURN_TIME_STEP) as usize) {
            let delta_time = time_after_start % BURN_TIME_STEP;
            closest_previous_point.next(delta_time, self.get_absolute_acceleration())
        } else {
            self.points.last().unwrap().clone()
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.get_time() > self.points.last().unwrap().get_time()
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        time - self.points.last().unwrap().get_time()
    }

    fn get_absolute_delta_v(&self) -> DVec2 {
        vec2(
            self.delta_v.x * self.tangent.x - self.delta_v.y * self.tangent.y,
            self.delta_v.x * self.tangent.y + self.delta_v.y * self.tangent.x)
    }

    fn get_absolute_acceleration(&self) -> DVec2 {
        self.get_absolute_delta_v().normalize() * BURN_ACCELERATION_MAGNITUDE
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.delta_v += adjustment;
        let start_point = self.get_start_point().clone();
        self.recompute_burn_points(start_point);
    }

    fn recompute_burn_points(&mut self, start_point: BurnPoint) {
        let mut points = vec![start_point.clone()];
        // We don't use a while loop because we need to compute at least 1 point (otherwise the duration of the burn is 0 which may break stuff)
        loop {
            let point = points.last().unwrap();
            if point.get_time() > start_point.get_time() + self.get_duration() {
                break;
            }
            points.push(point.next(BURN_TIME_STEP, self.get_absolute_acceleration()));
        }
        self.points = points;
    }

    pub fn reset(&mut self) {
        self.current_point = self.points.first().unwrap().clone();
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_point = self.get_point_at_time(self.current_point.get_time() + delta_time);
    }
}

#[cfg(test)]
mod test {
    use nalgebra_glm::vec2;

    use crate::{components::physics_component::trajectory::{brute_force_tester::BruteForceTester, burn::Burn}, storage::entity_allocator::Entity, constants::{BURN_ACCELERATION_MAGNITUDE, BURN_TIME_STEP}};

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
        assert!((tester.get_time() - burn.get_end_point().get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_end_point().get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_end_point().get_velocity()).magnitude() < 1.0);

        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        tester.update(0.5 * duration);
        assert!((tester.get_time() - burn.get_point_at_time(0.5 * duration).get_time()).abs() < 1.0);
        assert!((tester.get_position() - burn.get_point_at_time(0.5 * duration).get_position()).magnitude() < 1.0);
        assert!((tester.get_velocity() - burn.get_point_at_time(0.5 * duration).get_velocity()).magnitude() < 1.0);

        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration, BURN_TIME_STEP);
        tester.update(0.5 * duration);
        burn.update(0.5 * duration);
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