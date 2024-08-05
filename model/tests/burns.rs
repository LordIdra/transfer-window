use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{burn::rocket_equation_function::RocketEquationFunction, orbit::{builder::OrbitBuilder, orbit_direction::OrbitDirection, Orbit}, PathComponent}, vessel_component::{class::VesselClass, faction::Faction, timeline::{start_burn::StartBurnEvent, TimelineEvent}, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_burn_without_engine_or_fuel_tank() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None)));

    let vessel_component = VesselComponent::new(VesselClass::Station, Faction::Player);
    let orbit = OrbitBuilder {
        parent: earth,
        mass: vessel_component.mass(),
        parent_mass: earth_mass,
        rotation: 0.0,
        position: vec2(0.01041e9, 0.0),
        velocity: vec2(0.0, 8.250e3),
        time: 0.0,
    }.build();
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(vessel_component)
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    assert!(!StartBurnEvent::can_create_ever(&model, vessel));
}

#[test]
fn test_create_burn_with_zero_dv() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None)));

    let vessel_component = VesselComponent::new(VesselClass::Scout1, Faction::Player);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_path_component(PathComponent::new_with_orbit(Orbit::circle(earth, vessel_component.mass(), earth_mass, vec2(0.01041e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10)))
        .with_vessel_component(vessel_component));
    
    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));

    let time = 101.0;
    let mass_before = model.mass_at_time(vessel, time, None);
    let position_before = model.position_at_time(vessel, time, None);
    let velocity_before = model.velocity_at_time(vessel, time, None);

    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, time));
    model.add_event(vessel, event);

    let mass_after = model.mass_at_time(vessel, time, None);
    let position_after = model.position_at_time(vessel, time, None);
    let velocity_after = model.velocity_at_time(vessel, time, None);

    assert_eq!(model.path_component(vessel).final_burn().unwrap().total_dv(), 0.0);

    println!("mass after = {mass_after} mass before = {mass_before}");
    println!("position after = {position_after:?} position before = {position_before:?}");
    println!("velocity after = {velocity_after:?} velocity before = {velocity_before:?}");

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
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None)));

    let vessel_component = VesselComponent::new(VesselClass::Scout1, Faction::Player);
    let vessel_component_dry_mass = vessel_component.monopropellant_wet_mass();
    let vessel_component_fuel_mass = vessel_component.fuel_kg();
    let orbit = Orbit::circle(earth, vessel_component.mass(), earth_mass, vec2(0.01041e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(vessel_component.clone())
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));
    
    let burn_time = 100.0;
    let dv = vec2(150.0, 0.0);
    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, burn_time));
    model.add_event(vessel, event);
    model.burn_starting_at_time(vessel, burn_time); // just to make sure empty burns can be acquired
    model.timeline_event_at_time(vessel, burn_time)
        .as_start_burn()
        .unwrap()
        .adjust(&mut model, dv);

    let burn = model.burn_starting_at_time(vessel, burn_time);
    let start_time = burn.start_point().time();
    let end_time = burn.end_point().time();
    let duration = end_time - start_time;

    let velocity_before = model.velocity_at_time(vessel, start_time - 0.1, None);
    let velocity_after = model.velocity_at_time(vessel, end_time + 0.1, None);
    let actual_dv = (velocity_before.magnitude() - velocity_after.magnitude()).abs();
    println!("DV actual = {} expected = {}", actual_dv, dv.magnitude());
    assert!((actual_dv - dv.magnitude()).abs() < 0.4);

    let test_time = start_time + duration / 2.0;
    let mass_at_time = model.mass_at_time(vessel, test_time, None);
    let rocket_equation_function = RocketEquationFunction::new(
        vessel_component_dry_mass,
        vessel_component_fuel_mass,
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
        duration / 2.0);

    // *sigh* some variation is expected because the mass clamps to the lowest previous point
    // to avoid integration errors when interpolating points
    println!("actual mass = {} expected mass = {}", mass_at_time, rocket_equation_function.mass());
    assert!((mass_at_time - rocket_equation_function.mass()).abs() < 20.0);

    let mass_before = model.mass_at_time(vessel, start_time - 0.1, None);
    let mass_after = model.mass_at_time(vessel, end_time + 0.1, None);
    let rocket_equation_function = RocketEquationFunction::new(
        vessel_component_dry_mass,
        vessel_component_fuel_mass,
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
        duration);

    assert_eq!(mass_after, rocket_equation_function.mass());
    println!("mass before = {} mass after = {} expected burnt = {}", mass_before, mass_after, rocket_equation_function.fuel_burnt_kg());
    assert!(((mass_before - mass_after) - rocket_equation_function.fuel_burnt_kg()).abs() < 1.0e-3);
}