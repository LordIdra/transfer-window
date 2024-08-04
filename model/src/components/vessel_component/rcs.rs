use nalgebra_glm::{vec2, DVec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RcsThruster {
    throttle: f64,
    thrust: f64,
    position: DVec2,
    force_unit: DVec2,
}
impl RcsThruster {
    pub fn new(thrust: f64, position: DVec2, force_unit: DVec2) -> Self {
        Self { throttle: 0.0, thrust, position, force_unit }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcsControlScheme {
    thruster_throttles: Vec<f64>,
    force: Option<f64>,
    angular_force: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RcsControlSchemeType {
    Clockwise,
    Anticlockwise,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RcsControlSchemes {
    clockwise: RcsControlScheme,
    anticlockwise: RcsControlScheme,
}

impl RcsControlSchemes {
    pub fn get(&self, type_: RcsControlSchemeType) -> &RcsControlScheme {
        match type_ {
            RcsControlSchemeType::Clockwise => &self.clockwise,
            RcsControlSchemeType::Anticlockwise => &self.anticlockwise,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RcsType {
    LightRcs
}

impl RcsType {
    pub fn mass(self) -> f64 {
        match self {
            RcsType::LightRcs => 20.0,
        }
    }

    fn thrusters(self) -> Vec<RcsThruster> {
        match self {
            RcsType::LightRcs => vec![
                RcsThruster::new(1.0, vec2(10.0, 78.0), vec2(0.0, -1.0)),
                RcsThruster::new(1.0, vec2(10.0, 78.0), vec2(1.0, 0.0)),
                RcsThruster::new(1.0, vec2(10.0, 78.0), vec2(-1.0, 0.0)),

                RcsThruster::new(1.0, vec2(10.0, -78.0), vec2(0.0, 1.0)),
                RcsThruster::new(1.0, vec2(10.0, -78.0), vec2(1.0, 0.0)),
                RcsThruster::new(1.0, vec2(10.0, -78.0), vec2(-1.0, 0.0)),

                RcsThruster::new(1.0, vec2(-199.0, 88.0), vec2(0.0, -1.0)),
                RcsThruster::new(1.0, vec2(-199.0, 88.0), vec2(1.0, 0.0)),
                RcsThruster::new(1.0, vec2(-199.0, 88.0), vec2(-1.0, 0.0)),

                RcsThruster::new(1.0, vec2(-199.0, -88.0), vec2(0.0, 1.0)),
                RcsThruster::new(1.0, vec2(-199.0, -88.0), vec2(1.0, 0.0)),
                RcsThruster::new(1.0, vec2(-199.0, -88.0), vec2(-1.0, 0.0)),
            ],
        }
    }

    fn control_scheme(self) -> RcsControlSchemes {
        match self {
            RcsType::LightRcs => RcsControlSchemes {
                clockwise: RcsControlScheme {
                    force: None,
                    angular_force: Some(541.0),
                    thruster_throttles: vec![
                        0.0, 0.0, 1.0,
                        1.0, 1.0, 0.0,
                        1.0, 0.0, 1.0,
                        0.0, 1.0, 0.0
                    ],
                },

                anticlockwise: RcsControlScheme {
                    force: None,
                    angular_force: Some(-541.0),
                    thruster_throttles: vec![
                        1.0, 1.0, 0.0,
                        0.0, 0.0, 1.0,
                        0.0, 1.0, 0.0,
                        1.0, 0.0, 1.0
                    ],
                },
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rcs {
    rcs_type: RcsType,
    thrusters: Vec<RcsThruster>,
    control_scheme: RcsControlSchemes,
}

impl Rcs {
    pub fn new(rcs_type: RcsType) -> Self {
        Rcs { 
            rcs_type,
            thrusters: rcs_type.thrusters() ,
            control_scheme: rcs_type.control_scheme(),
        }
    }

    pub fn type_(&self) -> RcsType {
        self.rcs_type
    }

    pub fn control_scheme(&self, type_: RcsControlSchemeType) -> &RcsControlScheme {
        self.control_scheme.get(type_)
    }
}
