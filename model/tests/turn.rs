use std::f64::consts::PI;

use nalgebra_glm::vec2;
use transfer_window_model::{components::vessel_component::timeline::{start_burn::StartBurnEvent, start_turn::StartTurnEvent, TimelineEvent}, model::{state_query::StateQuery, Model}, test_util::{self, assert_float_equal}};

#[test]
fn test_cannot_turn_as_station() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let station = test_util::station_leo(&mut model, earth);

    assert!(!StartTurnEvent::can_create_ever(&model, station));
}

#[test]
fn test_turn_with_zero_dv() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);
    
    model.update(0.01);
    
    assert!(StartTurnEvent::can_create_ever(&model, vessel));

    let turn_event_time = 100.0;
    let measure_time = 140.0;
    let measure_snapshot = model.snapshot_at(measure_time);
    let mass_before = measure_snapshot.mass(vessel);
    let rotation_before = measure_snapshot.rotation(vessel);

    let event = TimelineEvent::StartTurn(StartTurnEvent::new(&mut model, vessel, turn_event_time));
    model.add_event(vessel, event);

    let mass_after = measure_snapshot.mass(vessel);
    let rotation_after = measure_snapshot.rotation(vessel);

    let event = model.start_turn_event_at_time(vessel, turn_event_time).unwrap();
    let turn_snapshot = model.snapshot_at(event.time());
    let turn = turn_snapshot.turn_starting_now(vessel);
    assert_float_equal(turn.fuel_burnt(), 0.0, 0.01);
    assert_float_equal(turn.duration(), 0.0, 0.01);
    
    println!("mass after = {mass_after} mass before = {mass_before}");
    println!("rotation after = {rotation_after:?} rotation before = {rotation_before:?}");
    
    assert_float_equal(rotation_after, rotation_before, 1.0e-5);
    assert_float_equal(mass_after, mass_before, 1.0e-5);
}

#[test]
fn test_create_and_adjust_turn() {
    let mut model = Model::default();

    let sun = test_util::sun(&mut model);
    let earth = test_util::earth(&mut model, sun);
    let vessel = test_util::test_ship_leo(&mut model, earth);

    model.update(0.01);
    
    assert!(StartTurnEvent::can_create_ever(&model, vessel));
    
    // Create and adjust turn
    let turn_event_time = 100.0;
    let rotation_change = PI / 2.0;
    let timeline_event = TimelineEvent::StartTurn(StartTurnEvent::new(&mut model, vessel, turn_event_time));
    model.add_event(vessel, timeline_event);
    model.start_turn_event_at_time(vessel, turn_event_time)
        .unwrap()
        .adjust(&mut model, rotation_change);

    // Get turn parameters
    let turn_snapshot = model.snapshot_at(turn_event_time);
    let turn = turn_snapshot.turn_starting_now(vessel);

    // Test change in rotation
    let rotation_before = model.snapshot_at(turn_event_time - 0.001).rotation(vessel);
    let rotation_after = model.snapshot_at(turn.end_point().time() + 0.001).rotation(vessel);
    let actual_rotation_change = (rotation_before - rotation_after).abs();
    
    assert_float_equal(rotation_change, actual_rotation_change, 0.01);

    // Test mass
    let vessel_component = model.vessel_component(vessel);
    let expected_burnt_mass = vessel_component.rcs().unwrap().turn_fuel_kg_per_second() * turn.duration();
    let expected_mass = vessel_component.mass() - expected_burnt_mass;
    let actual_mass = model.snapshot_at(turn.end_point().time()).mass(vessel);

    println!("actual mass = {actual_mass} expected mass = {expected_mass}");
    assert_float_equal(actual_mass, expected_mass, 1.0e-5);
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
    let burn_event_time = 100.0;
    let measurement_time = 120.0;
    let measurement_snapshot = model.snapshot_at(measurement_time);
    let position_before = measurement_snapshot.position(vessel);
    let velocity_before = measurement_snapshot.velocity(vessel);
    
    // Create, adjust, and cancel burn
    let dv = vec2(150.0, 0.0);
    let timeline_event = TimelineEvent::StartBurn(StartBurnEvent::new(&mut model, vessel, burn_event_time));
    model.add_event(vessel, timeline_event);
    model.start_burn_event_at_time(vessel, burn_event_time)
        .unwrap()
        .adjust(&mut model, dv);
    model.cancel_last_event(vessel);

    // Get parameters again
    let position_after = measurement_snapshot.position(vessel);
    let velocity_after = measurement_snapshot.velocity(vessel);
    
    // Check parameters have not changed
    assert_eq!(position_before, position_after);
    assert_eq!(velocity_before, velocity_after);
}

