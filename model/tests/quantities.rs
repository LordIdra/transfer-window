use std::f64::consts::PI;

use nalgebra_glm::vec2;
use transfer_window_model::{components::{orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, TrajectoryComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_stationary_position() {
    let mut model = Model::default();

    let earth_position = vec2(100.0, 0.0);
    let planet = model.allocate(EntityBuilder::default()
        .with_stationary_component(StationaryComponent::new(earth_position)));

    assert!(model.get_position(planet) == earth_position);
    assert!(model.get_absolute_position(planet) == earth_position);
}

#[test]
fn test_stationary_velocity() {
    let mut model = Model::default();

    let earth_position = vec2(100.0, 0.0);
    let planet = model.allocate(EntityBuilder::default()
        .with_stationary_component(StationaryComponent::new(earth_position)));

    assert!(model.get_velocity(planet) == vec2(0.0, 0.0));
    assert!(model.get_absolute_velocity(planet) == vec2(0.0, 0.0));
}

#[test]
fn test_trajectory_position() {
    let mut model = Model::default();

    let planet = model.allocate(EntityBuilder::default()
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let vessel_position = vec2(1.0e4, 0.0);
    let orbit = Orbit::circle(planet, 1.0e3, 1.0e16, vessel_position, 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let trajectory_component = TrajectoryComponent::default()
        .with_segment(Segment::Orbit(orbit.clone()));
    let vessel = model.allocate(EntityBuilder::default()
        .with_trajectory_component(trajectory_component));

    assert!(model.get_position(vessel) == vessel_position);
    assert!(model.get_absolute_position(vessel) == vessel_position);

    model.update(orbit.get_period().unwrap() / 4.0);
    
    let expected = vec2(-vessel_position.y, vessel_position.x);
    
    assert!((model.get_position(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);
    assert!((model.get_absolute_position(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);
}

#[test]
fn test_trajectory_velocity() {
    let mut model = Model::default();

    let planet = model.allocate(EntityBuilder::default()
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let vessel_position = vec2(1.0e4, 0.0);
    let orbit = Orbit::circle(planet, 1.0e3, 1.0e16, vessel_position, 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let trajectory_component = TrajectoryComponent::default()
        .with_segment(Segment::Orbit(orbit.clone()));
    let vessel = model.allocate(EntityBuilder::default()
        .with_trajectory_component(trajectory_component));

    let expected = orbit.get_velocity_from_theta(0.0);
    assert!((model.get_velocity(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);
    assert!((model.get_absolute_velocity(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);

    model.update(orbit.get_period().unwrap() / 4.0);
    
    let expected_theta = PI / 2.0;
    let expected = orbit.get_velocity_from_theta(expected_theta);
    
    assert!((model.get_velocity(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);
    assert!((model.get_absolute_velocity(vessel) - expected).magnitude() / expected.magnitude() < 1.0e-3);
}

#[test]
fn test_simple_mass() {
    let mut model = Model::default();

    let mass = 1.0e23;
    let planet = model.allocate(EntityBuilder::default()
        .with_orbitable_component(OrbitableComponent::new(mass, 1.0e4)));

    assert_eq!(model.get_mass(planet), mass);
}