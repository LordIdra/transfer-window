use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}, vessel_component::{system_slot::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, Slot, SlotLocation}, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_burn_without_engine_or_fuel_tank() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let class = VesselClass::Light;
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(class))
        .with_trajectory_component(TrajectoryComponent::default()));
    let orbit = Orbit::new(earth, class.get_mass(), earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0);
    model.get_trajectory_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    assert!(!model.can_create_burn(vessel));

    let vessel_component = model.get_vessel_component_mut(vessel).get_slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::Efficient)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;

    assert!(!model.can_create_burn(vessel));

    let vessel_component = model.get_vessel_component_mut(vessel).get_slots_mut();
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
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let class = VesselClass::Light;
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(class))
        .with_trajectory_component(TrajectoryComponent::default()));
    let orbit = Orbit::new(earth, class.get_mass(), earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0).with_end_at(1.0e10);
    model.get_trajectory_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    model.update(0.01);

    let vessel_component = model.get_vessel_component_mut(vessel).get_slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::Efficient)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;
    let fuel_tank = Slot::FuelTank(Some(FuelTank::new(FuelTankType::Medium)));
    *vessel_component.get_mut(SlotLocation::Middle) = fuel_tank;
    assert!(model.can_create_burn(vessel));

    let time = 1200.0;
    let position_at_time_before_creating_burn = model.get_position_at_time(vessel, time);
    let velocity_at_time_before_creating_burn = model.get_velocity_at_time(vessel, time);
    model.create_burn(vessel, 100.0);
    let position_at_time_after_creating_burn = model.get_position_at_time(vessel, time);
    let velocity_at_time_after_creating_burn = model.get_velocity_at_time(vessel, time);

    assert!((position_at_time_after_creating_burn - position_at_time_before_creating_burn).magnitude() < 1.0e-1);
    assert!((velocity_at_time_after_creating_burn - velocity_at_time_before_creating_burn).magnitude() < 1.0e-3);
}