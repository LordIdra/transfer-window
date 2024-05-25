use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{burn::rocket_equation_function::RocketEquationFunction, orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::{system_slot::{engine::EngineType, fuel_tank::FuelTankType, Slot, SlotLocation}, timeline::{start_burn::BurnEvent, TimelineEvent}, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

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

    let engine = Slot::new_engine(EngineType::Efficient);
    model.set_slot(vessel, SlotLocation::Back, engine);

    assert!(!model.can_create_burn(vessel));

    let fuel_tank = Slot::new_fuel_tank(FuelTankType::Medium);
    model.set_slot(vessel, SlotLocation::Middle, fuel_tank);

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

    let engine = Slot::new_engine(EngineType::Efficient);
    model.set_slot(vessel, SlotLocation::Back, engine);
    let fuel_tank = Slot::new_fuel_tank(FuelTankType::Medium);
    model.set_slot(vessel, SlotLocation::Middle, fuel_tank);
    assert!(model.can_create_burn(vessel));

    let time = 101.0;
    let mass_before = model.mass_at_time(vessel, time);
    let position_before = model.position_at_time(vessel, time);
    let velocity_before = model.velocity_at_time(vessel, time);

    let event = TimelineEvent::Burn(BurnEvent::new(&mut model, vessel, time));
    model.add_event(vessel, event);

    let mass_after = model.mass_at_time(vessel, time);
    let position_after = model.position_at_time(vessel, time);
    let velocity_after = model.velocity_at_time(vessel, time);

    assert_eq!(model.path_component(vessel).final_burn().unwrap().total_dv(), 0.0);

    println!("mass after = {} mass before = {}", mass_after, mass_before);
    println!("position after = {:?} position before = {:?}", position_after, position_before);
    println!("velocity after = {:?} velocity before = {:?}", velocity_after, velocity_before);

    assert!((mass_after - mass_before).abs() < 1.0e-5);
    assert!((position_after - position_before).magnitude() < 1.0e-1);
    assert!((velocity_after - velocity_before).magnitude() < 1.0e-3);
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

    let engine_type = EngineType::HighThrust;
    let engine = Slot::new_engine(engine_type.clone());
    model.set_slot(vessel, SlotLocation::Back, engine);
    let fuel_tank_type = FuelTankType::Medium;
    let fuel_tank = Slot::new_fuel_tank(fuel_tank_type.clone());
    model.set_slot(vessel, SlotLocation::Middle, fuel_tank);
    assert!(model.can_create_burn(vessel));
    
    let burn_time = 100.0;
    let dv = vec2(150.0, 0.0);
    let event = TimelineEvent::Burn(BurnEvent::new(&mut model, vessel, burn_time));
    model.add_event(vessel, event);
    model.burn_starting_at_time(vessel, burn_time); // just to make sure empty burns can be acquired
    model.event_at_time(vessel, burn_time)
        .as_burn()
        .unwrap()
        .adjust(&mut model, dv);

    let burn = model.burn_starting_at_time(vessel, burn_time);
    let start_time = burn.start_point().time();
    let end_time = burn.end_point().time();
    let duration = end_time - start_time;

    let velocity_before = model.velocity_at_time(vessel, start_time - 0.1);
    let velocity_after = model.velocity_at_time(vessel, end_time + 0.1);
    let actual_dv = (velocity_before.magnitude() - velocity_after.magnitude()).abs();
    println!("DV actual = {} expected = {}", actual_dv, dv.magnitude());
    assert!((actual_dv - dv.magnitude()) < 1.0e-3);

    let test_time = start_time + duration / 2.0;
    let mass_at_time = model.mass_at_time(vessel, test_time);
    let rocket_equation_function = RocketEquationFunction::new(
        class.mass(),
        fuel_tank_type.capacity_kg(),
        engine_type.fuel_kg_per_second(),
        engine_type.specific_impulse_space(),
        duration / 2.0);
    assert_eq!(mass_at_time, rocket_equation_function.mass());
    println!("actual mass = {} expected mass = {}", mass_at_time, rocket_equation_function.mass());
    assert!((mass_at_time - rocket_equation_function.mass()).abs() < 1.0e-3);

    let mass_before = model.mass_at_time(vessel, start_time - 0.1);
    let mass_after = model.mass_at_time(vessel, end_time + 0.1);
    let rocket_equation_function = RocketEquationFunction::new(
        class.mass(),
        fuel_tank_type.capacity_kg(),
        engine_type.fuel_kg_per_second(),
        engine_type.specific_impulse_space(),
        duration);
    assert_eq!(mass_after, rocket_equation_function.mass());
    println!("mass before = {} mass after = {} expected burnt = {}", mass_before, mass_after, rocket_equation_function.fuel_kg_burnt());
    assert!(((mass_before - mass_after) - rocket_equation_function.fuel_kg_burnt()).abs() < 1.0e-3);
}