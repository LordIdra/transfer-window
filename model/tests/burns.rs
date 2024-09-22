use nalgebra_glm::vec2;
use transfer_window_model::{components::{path_component::rocket_equation_function::RocketEquationFunction, vessel_component::{faction::Faction, timeline::{start_burn::StartBurnEvent, TimelineEvent}}}, test_util::{self, assert_dvec_equal, assert_float_equal}, Model};

#[test]
fn test_cannot_burn_as_station() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let station = test_util::station_leo(&mut model, earth);

    assert!(!StartBurnEvent::can_create_ever(&model, station));
}

#[test]
fn test_burn_with_zero_dv() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);
    
    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));

    let burn_time = 100.0;
    let measure_time = burn_time + 1.0;

    let mass_before = model.mass_at_time(vessel, measure_time, None);
    let position_before = model.position_at_time(vessel, measure_time, None);
    let velocity_before = model.velocity_at_time(vessel, measure_time, None);

    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, burn_time));
    model.add_event(vessel, event);

    let mass_after = model.mass_at_time(vessel, measure_time, None);
    let position_after = model.position_at_time(vessel, measure_time, None);
    let velocity_after = model.velocity_at_time(vessel, measure_time, None);

    let burn = model.burn_starting_at_time(vessel, burn_time);
    assert_eq!(burn.total_dv(), 0.0);
    
    println!("mass after = {mass_after} mass before = {mass_before}");
    println!("position after = {position_after:?} position before = {position_before:?}");
    println!("velocity after = {velocity_after:?} velocity before = {velocity_before:?}");
    
    assert_float_equal(mass_after, mass_before, 1.0e-5);
    assert_dvec_equal(position_after, position_before, 1.0);
    assert_dvec_equal(velocity_after, velocity_before, 1.0e-5);
}

#[test]
fn test_create_and_adjust_burn() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);

    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));
    
    // Create and adjust burn
    let burn_start_time = 100.0;
    let dv = vec2(150.0, 0.0);
    let timeline_event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, burn_start_time));
    model.add_event(vessel, timeline_event);
    model.start_burn_event_at_time(vessel, burn_start_time)
        .unwrap()
        .adjust(&mut model, dv);

    // Get burn parameters
    let burn = model.burn_starting_at_time(vessel, burn_start_time);
    let burn_end_time = burn.end_point().time();
    let duration = burn.duration();

    // Test change in velocity
    let velocity_before = model.velocity_at_time(vessel, burn_start_time - 0.001, None);
    let velocity_after = model.velocity_at_time(vessel, burn_end_time + 0.001, None);
    let actual_dv = (velocity_before.magnitude() - velocity_after.magnitude()).abs();
    
    // Some variation is expected due to gravity
    assert_float_equal(dv.magnitude(), actual_dv, 1.0);

    // Test mass at halfway point
    let vessel_component = model.vessel_component(vessel);

    let mass_test_time = burn_start_time + duration / 2.0;
    let actual_mass = model.mass_at_time(vessel, mass_test_time, None);
    let expected_mass = RocketEquationFunction::new(
        vessel_component.dry_mass(),
        vessel_component.fuel_kg(),
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
    ).step_by_time(duration / 2.0).unwrap().mass();

    println!("actual mass = {actual_mass} expected mass = {expected_mass}");
    assert_float_equal(actual_mass, expected_mass, 1.0e-5);

    // Test mass
    let start_mass = model.mass_at_time(vessel, burn_start_time - 0.1, None);
    let end_mass = model.mass_at_time(vessel, burn_end_time + 0.1, None);
    let start_rocket_equation_function = RocketEquationFunction::new(
        vessel_component.dry_mass(),
        vessel_component.fuel_kg(),
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
    );
    let end_rocket_equation_function = start_rocket_equation_function.step_by_time(duration).unwrap();

    assert_float_equal(expected_mass, actual_mass, 1.0e-5);

    // Test fuel burnt
    let actual_fuel_burnt = start_mass - end_mass;
    let expected_fuel_burnt = start_rocket_equation_function.mass() - end_rocket_equation_function.mass();

    assert_float_equal(expected_fuel_burnt, actual_fuel_burnt, 1.0e-5);
}

#[test]
fn test_cancel_burn() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);

    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));

    // Get parameters
    let burn_time = 100.0;
    let measurement_time = burn_time + 10.0;
    let position_before = model.position_at_time(vessel, measurement_time, Some(Faction::Player));
    let velocity_before = model.velocity_at_time(vessel, measurement_time, Some(Faction::Player));
    
    // Create, adjust, and cancel burn
    let dv = vec2(150.0, 0.0);
    let timeline_event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, burn_time));
    model.add_event(vessel, timeline_event);
    model.start_burn_event_at_time(vessel, burn_time)
        .unwrap()
        .adjust(&mut model, dv);
    model.cancel_last_event(vessel);

    // Get parameters again
    let position_after = model.position_at_time(vessel, measurement_time, Some(Faction::Player));
    let velocity_after = model.velocity_at_time(vessel, measurement_time, Some(Faction::Player));
    
    // Check parameters have not changed
    assert_eq!(position_before, position_after);
    assert_eq!(velocity_before, velocity_after);
}

#[test]
fn test_burn_over_max_dv() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);

    model.update(0.01);
    
    assert!(StartBurnEvent::can_create_ever(&model, vessel));

    // Get parameters
    let burn_start_time = 100.0;
    
    // Create and adjust burn
    let dv = vec2(model.end_dv(vessel).unwrap() + 0.1, 0.0);
    let timeline_event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, vessel, burn_start_time));
    model.add_event(vessel, timeline_event);
    model.start_burn_event_at_time(vessel, burn_start_time)
        .unwrap()
        .adjust(&mut model, dv);
    
    // Get burn parameters
    let burn = model.burn_starting_at_time(vessel, burn_start_time);
    let burn_end_time = burn.end_point().time();
    let duration = burn.duration();

    // Test change in velocity
    let velocity_before = model.velocity_at_time(vessel, burn_start_time - 0.001, None);
    let velocity_after = model.velocity_at_time(vessel, burn_end_time + 0.001, None);
    let actual_dv = (velocity_before.magnitude() - velocity_after.magnitude()).abs();
    
    // Some variation is expected due to integration errors
    assert_float_equal(dv.magnitude(), actual_dv, 20.0);

    // Test mass at halfway point
    let vessel_component = model.vessel_component(vessel);

    let mass_test_time = burn_start_time + duration / 2.0;
    let actual_mass = model.mass_at_time(vessel, mass_test_time, None);
    let expected_mass = RocketEquationFunction::new(
        vessel_component.dry_mass(),
        vessel_component.fuel_kg(),
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
    ).step_by_time(duration / 2.0).unwrap().mass();

    println!("actual mass = {actual_mass} expected mass = {expected_mass}");
    assert_float_equal(actual_mass, expected_mass, 1.0e-5);

    // Test mass
    let start_mass = model.mass_at_time(vessel, burn_start_time - 0.1, None);
    let end_mass = model.mass_at_time(vessel, burn_end_time + 0.1, None);
    let start_rocket_equation_function = RocketEquationFunction::new(
        vessel_component.dry_mass(),
        vessel_component.fuel_kg(),
        vessel_component.fuel_kg_per_second(),
        vessel_component.specific_impulse().unwrap(),
    );
    let end_rocket_equation_function = start_rocket_equation_function.step_by_time(duration).unwrap();

    assert_float_equal(expected_mass, actual_mass, 1.0e-5);

    // Test fuel burnt
    let actual_fuel_burnt = start_mass - end_mass;
    let expected_fuel_burnt = start_rocket_equation_function.mass() - end_rocket_equation_function.mass();

    assert_float_equal(expected_fuel_burnt, actual_fuel_burnt, 1.0e-5);
}

