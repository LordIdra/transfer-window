use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, Slot, SlotLocation}, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_burn_without_engine_or_fuel_tank() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    let class = VesselClass::Light;
    let orbit = Orbit::new(earth, class.mass(), earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(class))
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    assert!(!model.can_create_burn(vessel));

    let vessel_component = model.vessel_component_mut(vessel).slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::Efficient)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;

    assert!(!model.can_create_burn(vessel));

    let vessel_component = model.vessel_component_mut(vessel).slots_mut();
    let fuel_tank = Slot::FuelTank(Some(FuelTank::new(FuelTankType::Medium)));
    *vessel_component.get_mut(SlotLocation::Middle) = fuel_tank;

    assert!(model.can_create_burn(vessel));
}

#[test]
fn test_create_burn_with_zero_dv() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    let class = VesselClass::Light;
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(class))
        .with_path_component(PathComponent::default()));
    let orbit = Orbit::circle(earth, class.mass(), earth_mass, vec2(0.01041e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    model.path_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    model.update(0.01);

    let vessel_component = model.vessel_component_mut(vessel).slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::Efficient)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;
    let fuel_tank = Slot::FuelTank(Some(FuelTank::new(FuelTankType::Medium)));
    *vessel_component.get_mut(SlotLocation::Middle) = fuel_tank;
    assert!(model.can_create_burn(vessel));

    let time = 101.0;
    let mass_before = model.mass_at_time(vessel, time);
    let position_before = model.position_at_time(vessel, time);
    let velocity_before = model.velocity_at_time(vessel, time);
    model.create_burn(vessel, 100.0);
    let mass_after = model.mass_at_time(vessel, time);
    let position_after = model.position_at_time(vessel, time);
    let velocity_after = model.velocity_at_time(vessel, time);

    assert_eq!(model.path_component(vessel).final_burn().unwrap().total_dv(), 0.0);

    println!("mass after = {} mass before = {}", mass_after, mass_before);
    println!("position after = {:?} position before = {:?}", position_after, position_before);
    println!("velocity after = {:?} velocity before = {:?}", velocity_after, velocity_before);

    assert!((mass_after - mass_before).abs() < 1.0e-5);
    // TODO re-introduce these assertions once segments are reworked
    // assert!((position_after - position_before).magnitude() < 1.0e-1);
    // assert!((velocity_after - velocity_before).magnitude() < 1.0e-3);
}

#[test]
fn test_create_and_adjust_burn() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    let class = VesselClass::Light;
    let orbit = Orbit::circle(earth, class.mass(), earth_mass, vec2(0.01041e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(class))
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    model.update(0.01);

    let vessel_component = model.vessel_component_mut(vessel).slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::HighThrust)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;
    let fuel_tank = Slot::FuelTank(Some(FuelTank::new(FuelTankType::Medium)));
    *vessel_component.get_mut(SlotLocation::Middle) = fuel_tank;
    assert!(model.can_create_burn(vessel));

    model.create_burn(vessel, 100.0);
    model.adjust_burn(vessel, 100.0, vec2(150.0, 0.0));
    let start_time = model.path_component(vessel).final_burn().unwrap().start_point().time();
    let end_time = model.path_component(vessel).final_burn().unwrap().end_point().time();
    let velocity_before = model.velocity_at_time(vessel, start_time - 1.0);
    let velocity_after = model.velocity_at_time(vessel, end_time + 1.0);

    assert!(((velocity_before.magnitude() - velocity_after.magnitude()).abs() - 150.0) < 1.0e-3);
}