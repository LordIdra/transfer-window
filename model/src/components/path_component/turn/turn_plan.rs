use serde::{Deserialize, Serialize};
use transfer_window_common::angular_distance;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnPlan {
    angular_acceleration: f64,
    fuel_kg_per_second: f64,
    duration: f64,
    turn_direction: f64,
    dry_mass: f64,
    start_fuel_kg: f64,
    start_time: f64,
    start_rotation: f64,
    end_rotation: f64,
}

impl TurnPlan {
    /// Calculations for turning a ship to face a direction, with zero velocity at the start and
    /// end. Assumes constant angular acceleration.
    pub fn new(angular_acceleration: f64, fuel_kg_per_second: f64, dry_mass: f64, start_fuel_kg: f64, start_time: f64, start_rotation: f64, end_rotation: f64) -> Self {
        let turn_angle = angular_distance(start_rotation, end_rotation);
        let turn_direction = turn_angle.signum();
        let duration = 2.0 * f64::sqrt(turn_angle.abs() / angular_acceleration);
        Self { angular_acceleration, fuel_kg_per_second, duration, turn_direction, dry_mass, start_fuel_kg, start_time, start_rotation, end_rotation }
    }

    pub fn angular_acceleration(&self) -> f64 {
        self.angular_acceleration
    }
    
    pub fn fuel_kg_per_second(&self) -> f64 {
        self.fuel_kg_per_second
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn start_mass(&self) -> f64 {
        self.dry_mass + self.start_fuel_kg
    }

    pub fn start_fuel_kg(&self) -> f64 {
        self.start_fuel_kg
    }

    pub fn start_time(&self) -> f64 {
        self.start_time
    }

    pub fn start_rotation(&self) -> f64 {
        self.start_rotation
    }

    pub fn end_mass(&self) -> f64 {
        self.start_mass() - self.duration * self.fuel_kg_per_second
    }

    pub fn end_fuel_kg(&self) -> f64 {
        self.start_fuel_kg() - self.duration * self.fuel_kg_per_second
    }

    pub fn end_time(&self) -> f64 {
        self.start_time + self.duration
    }

    pub fn end_rotation(&self) -> f64 {
        self.end_rotation
    }

    pub fn mass_at_time(&self, time: f64) -> f64 {
        let time_since_start = time - self.start_time;
        self.start_mass() - time_since_start * self.fuel_kg_per_second
    }

    pub fn fuel_kg_at_time(&self, time: f64) -> f64 {
        let time_since_start = time - self.start_time;
        self.start_fuel_kg() - time_since_start * self.fuel_kg_per_second
    }

    pub fn rotation_at_time(&self, time: f64) -> f64 {
        let time_since_start = time - self.start_time;
        let additional_angle = if time_since_start < self.duration / 2.0 {
            0.5 * self.angular_acceleration * time_since_start.powi(2)
        } else {
             0.5 * self.angular_acceleration * (self.duration / 2.0).powi(2)
                + self.angular_acceleration * (self.duration / 2.0) * (time_since_start - self.duration / 2.0)
                - 0.5 * self.angular_acceleration * (time_since_start - self.duration / 2.0).powi(2)
        };
        self.start_rotation + self.turn_direction * additional_angle
    }

    pub fn angular_velocity_at_time(&self, time: f64) -> f64 {
        let time_since_start = time - self.start_time;
        if time_since_start < self.duration / 2.0 {
            time_since_start * self.angular_acceleration
        } else {
            (self.duration - time_since_start) * self.angular_acceleration
        }
    }

    pub fn dry_mass(&self) -> f64 {
        self.dry_mass
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::test_util::assert_float_equal;

    use super::TurnPlan;

    #[test]
    pub fn test_turn_plan() {
        let plan = TurnPlan::new(0.01, 0.2, 500.0, 1000.0, 250.0, PI / 4.0, PI / 3.0);
        assert_float_equal(0.0, plan.angular_velocity_at_time(plan.end_time()), 1.0e-5);
        assert_float_equal(0.04, plan.angular_velocity_at_time(plan.start_time() + 4.0), 1.0e-5);
        assert_float_equal(0.04, plan.angular_velocity_at_time(plan.end_time() - 4.0), 1.0e-5);
        assert_float_equal(plan.end_rotation, plan.rotation_at_time(plan.end_time()), 1.0e-5);
        assert_float_equal(PI / 4.0 + (PI / 3.0 - PI / 4.0) / 2.0, plan.rotation_at_time(plan.start_time() + plan.duration() / 2.0), 1.0e-5);
    }
}
