use nalgebra_glm::DVec2;

use super::orbit::scary_math::GRAVITATIONAL_CONSTANT;

pub struct BruteForceTester {
    parent_mass: f64,
    position: DVec2,
    velocity: DVec2,
    acceleration_from_time: Box<dyn Fn(f64) -> DVec2 + 'static>,
    time: f64,
    time_step: f64,
}

impl BruteForceTester {
    pub fn new(parent_mass: f64, position: DVec2, velocity: DVec2, acceleration_from_time: impl Fn(f64) -> DVec2 + 'static, time_step: f64) -> Self {
        let acceleration_from_time = Box::new(acceleration_from_time);
        Self { parent_mass, position, velocity, acceleration_from_time, time: 0.0, time_step }
    }

    fn step(&mut self, dt: f64) {
        let gravity_acceleration = -self.position.normalize() * GRAVITATIONAL_CONSTANT * self.parent_mass / self.position.magnitude_squared();
        let acceleration = (self.acceleration_from_time)(self.time) + gravity_acceleration;
        self.time += dt;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn update(&mut self, duration: f64) {
        let end_time = self.time + duration;
        while self.time + self.time_step < end_time {
            self.step(self.time_step);
        }
        self.step(end_time - self.time);
    }

    pub fn position(&self) -> DVec2 {
        self.position
    }

    pub fn velocity(&self) -> DVec2 {
        self.velocity
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}

mod test {
    use nalgebra_glm::vec2;

    use super::BruteForceTester;

    #[test]
    fn test_moon() {
        let parent_mass = 5.972e24; // earth's mass
        let start_position = vec2(3.633e8, 0.0);
        let start_velocity = vec2(0.0, 1.082e3);
        let period = 27.9917 * 24.0 * 60.0 * 60.0; 
        let acceleration_from_time = |_: f64| vec2(0.0, 0.0);
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time, 1.0);

        tester.update(0.5 * period);
        let expected_time = 0.5 * period;
        let expected_position = vec2(-4.155e8, 0.0);
        let expected_velocity = vec2(0.0, -0.945e3);
        assert!((tester.time() - expected_time).abs() < 2.0);
        assert!((tester.position() - expected_position).magnitude() < 1.0e5);
        assert!((tester.velocity() - expected_velocity).magnitude() < 1.0);

        tester.update(0.5 * period);
        let expected_time = period;
        let expected_position = start_position;
        let expected_velocity = start_velocity;
        assert!((tester.time() - expected_time).abs() < 2.0);
        assert!((tester.position() - expected_position).magnitude() < 1.0e5);
        assert!((tester.velocity() - expected_velocity).magnitude() < 1.0);
    }

    #[test]
    fn test_constant_acceleration() {
        let parent_mass = 1.0;
        let start_position = vec2(0.0, 1.0e-3); // must be nonzero otherwise force of gravity becomes infinite
        let start_velocity = vec2(0.0, 0.0);
        let constant_acceleration  = vec2(2.0, 0.0);
        let acceleration_from_time = move |_: f64| constant_acceleration;
        let duration = 120.0;
        let mut tester = BruteForceTester::new(parent_mass, start_position, start_velocity, acceleration_from_time, 0.01);

        tester.update(duration);
        let expected_time = duration;
        let expected_position = vec2(0.5 * constant_acceleration.magnitude() * duration.powi(2), 0.0);
        let expected_velocity = vec2(constant_acceleration.magnitude() * duration, 0.0);
        assert!((tester.time() - expected_time).abs() < 1.0e-1);
        assert!((tester.position() - expected_position).magnitude() < 2.0);
        assert!((tester.velocity() - expected_velocity).magnitude() < 1.0e-2);
    }
}
