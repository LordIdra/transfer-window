use nalgebra_glm::{vec2, DVec2};

use crate::{components::{orbitable_component::{builder::OrbitablePhysicsBuilder, OrbitableType}, path_component::orbit::{builder::InitialOrbitBuilder, orbit_direction::OrbitDirection}, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, model::Model, storage::{entity_allocator::Entity, entity_builder::{OrbitableBuilder, VesselBuilder}}};

pub const SUN_MASS: f64 = 1.989e30;
pub const SUN_RADIUS: f64 = 6.95508e8;
pub const SUN_ROTATION_PERIOD: f64 = 60.0 * 60.0;

pub const EARTH_MASS: f64 = 5.972e24;
pub const EARTH_RADIUS: f64 = 6.371e6;
pub const EARTH_ROTATION_PERIOD: f64 = 24.0 * 60.0 * 60.0;
pub const EARTH_PERIAPSIS: f64 = 147.095e9;
pub const EARTH_MAX_SPEED: f64 = 30.29e3;

pub const MOON_MASS: f64 = 7.348e22;
pub const MOON_RADIUS: f64 = 1.737e6;
pub const MOON_ROTATION_PERIOD: f64 = 27.0 * 24.0 * 60.0 * 60.0;
pub const MOON_PERIAPSIS: f64 = 0.3633e6;
pub const MOON_MAX_SPEED: f64 = 1.082e3;

pub fn sun(model: &mut Model) -> Entity {
    OrbitableBuilder {
        name: "Sun",
        mass: SUN_MASS,
        radius: SUN_RADIUS,
        rotation_period: SUN_ROTATION_PERIOD,
        rotation_angle: 0.0,
        type_: OrbitableType::Star,
        atmosphere: None,
        physics: OrbitablePhysicsBuilder::Stationary(vec2(0.0, 0.0)),
    }.build(model)
}

pub fn earth(model: &mut Model, sun: Entity) -> Entity {
    OrbitableBuilder {
        name: "Earth",
        mass: EARTH_MASS,
        radius: EARTH_RADIUS,
        rotation_period: EARTH_ROTATION_PERIOD,
        rotation_angle: 0.0,
        type_: OrbitableType::Planet,
        atmosphere: None,
        physics: OrbitablePhysicsBuilder::Orbit(InitialOrbitBuilder::Freeform { 
            parent: sun, 
            distance: EARTH_PERIAPSIS,
            angle: 0.0,
            direction: OrbitDirection::AntiClockwise,
            speed: EARTH_MAX_SPEED,
        }),
    }.build(model)
}

pub fn moon(model: &mut Model, sun: Entity) -> Entity {
    OrbitableBuilder {
        name: "Earth",
        mass: MOON_MASS,
        radius: MOON_RADIUS,
        rotation_period: MOON_ROTATION_PERIOD,
        rotation_angle: 0.0,
        type_: OrbitableType::Planet,
        atmosphere: None,
        physics: OrbitablePhysicsBuilder::Orbit(InitialOrbitBuilder::Freeform { 
            parent: sun, 
            distance: MOON_PERIAPSIS,
            angle: 0.0,
            direction: OrbitDirection::AntiClockwise,
            speed: MOON_MAX_SPEED,
        }),
    }.build(model)
}

pub fn station_leo(model: &mut Model, earth: Entity) -> Entity {
    VesselBuilder {
        name: "Station",
        vessel_component: VesselComponent::new(VesselClass::Station, Faction::Player),
        orbit_builder: InitialOrbitBuilder::Circular {
            parent: earth ,
            distance: 0.01e9,
            angle: 0.0,
            direction: OrbitDirection::AntiClockwise,
        }
    }.build(model)
}

pub fn scout_leo(model: &mut Model, earth: Entity) -> Entity {
    VesselBuilder {
        name: "Scout",
        vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
        orbit_builder: InitialOrbitBuilder::Circular {
            parent: earth ,
            distance: 0.011e9,
            angle: 0.0,
            direction: OrbitDirection::AntiClockwise,
        }
    }.build(model)
}

pub fn test_ship_leo(model: &mut Model, earth: Entity) -> Entity {
    VesselBuilder {
        name: "Test Ship",
        vessel_component: VesselComponent::new(VesselClass::TestShip, Faction::Player),
        orbit_builder: InitialOrbitBuilder::Circular {
            parent: earth ,
            distance: 0.012e9,
            angle: 0.0,
            direction: OrbitDirection::AntiClockwise,
        }
    }.build(model)
}

pub fn assert_float_equal(x1: f64, x2: f64, tolerance: f64) {
    let difference = (x1 - x2).abs();
    assert!(difference < tolerance, "{x1} != {x2} | tolerance: {tolerance} | difference: {difference}");
}

pub fn assert_dvec_equal(x1: DVec2, x2: DVec2, tolerance: f64) {
    let difference = (x1 - x2).magnitude();
    assert!(difference < tolerance, "{x1:?} != {x2:?} | tolerance: {tolerance} | difference: {difference}");
}
