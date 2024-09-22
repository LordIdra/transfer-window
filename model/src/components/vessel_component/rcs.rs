use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcsThruster {
    throttle: f64,
    thrust: f64,
    fuel_kg_per_second: f64,
    position: DVec2,
    force_unit: DVec2,
}
impl RcsThruster {
    pub fn new(thrust: f64, fuel_kg_per_second: f64, position: DVec2, force_unit: DVec2) -> Self {
        Self { throttle: 0.0, thrust, fuel_kg_per_second, position, force_unit }
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        self.fuel_kg_per_second
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcsControlScheme {
    force: Option<f64>,
    angular_force: Option<f64>,
    thruster_throttles_positive: Vec<f64>,
    thruster_throttles_negative: Vec<f64>,
}

impl RcsControlScheme {
    pub fn new(force: Option<f64>, angular_force: Option<f64>, thruster_throttles_positive: Vec<f64>, thruster_throttles_negative: Vec<f64>) -> Self {
        Self { force, angular_force, thruster_throttles_positive, thruster_throttles_negative  }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rcs {
    mass: f64,
    thrusters: Vec<RcsThruster>,
    turn: RcsControlScheme,
}

impl Rcs {
    pub fn new(mass: f64, thrusters: Vec<RcsThruster>, turn: RcsControlScheme) -> Self {
        Rcs { mass, thrusters, turn }
    }

    pub fn turn_fuel_kg_per_second(&self) -> f64 {
        let mut fuel_use = 0.0;
        for i in 0..self.thrusters.len() {
            fuel_use += self.thrusters[i].fuel_kg_per_second() * self.turn.thruster_throttles_positive[i];
        }
        fuel_use
    }

    pub fn turn_force(&self) -> f64 {
        self.turn.angular_force.expect("Turn scheme must specify an angular force")
    }
}
