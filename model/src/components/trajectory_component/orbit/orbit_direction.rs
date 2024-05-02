use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use super::scary_math::transverse_velocity;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum OrbitDirection {
    AntiClockwise,
    Clockwise,
}

impl OrbitDirection {
    pub fn new(position: DVec2, velocity: DVec2) -> Self {
        if transverse_velocity(position, velocity).is_sign_positive() {
            Self::AntiClockwise
        } else {
            Self::Clockwise
        }
    }
}

impl OrbitDirection {
    pub fn is_anticlockwise(self) -> bool {
        matches!(self, Self::AntiClockwise)
    }

    pub fn is_clockwise(self) -> bool {
        matches!(self, Self::Clockwise)
    }
}

#[test]
fn test() {
    use nalgebra_glm::vec2;

    assert_eq!(OrbitDirection::new(vec2(1.0, 0.0), vec2(0.0, 1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::new(vec2(1.0, 0.0), vec2(0.0, -1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::new(vec2(1.0, 1.0), vec2(0.0, 1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::new(vec2(1.0, 1.0), vec2(0.0, -1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::new(vec2(-1.0, 1.0), vec2(0.0, -1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::new(vec2(-1.0, 1.0), vec2(0.0, 1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::new(vec2(-0.2, 1.0), vec2(1.0, 0.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::new(vec2(-1.0, 1.0), vec2(-1.0, 0.0)), OrbitDirection::AntiClockwise);
}