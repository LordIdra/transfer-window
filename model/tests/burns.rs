use transfer_window_model::{components::vessel_component::timeline::{start_burn::StartBurnEvent, TimelineEvent}, model::{state_query::StateQuery, Model}, test_util::{self, assert_dvec_equal, assert_float_equal}};

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

    let burn_event_time = 100.0;
    let measure_time = 140.0;
    let measure_snapshot = model.snapshot_at(measure_time);
    let mass_before = measure_snapshot.mass(vessel);
    let position_before = measure_snapshot.position(vessel);
    let velocity_before = measure_snapshot.velocity(vessel);
    let rotation_before = measure_snapshot.rotation(vessel);

    let event = TimelineEvent::StartBurn(StartBurnEvent::new(&mut model, vessel, burn_event_time));
    model.add_event(vessel, event);

    let mass_after = measure_snapshot.mass(vessel);
    let position_after = measure_snapshot.position(vessel);
    let velocity_after = measure_snapshot.velocity(vessel);
    let rotation_after = measure_snapshot.rotation(vessel);

    let event = model.start_burn_event_at_time(vessel, burn_event_time).unwrap();
    let burn_snapshot = model.snapshot_at(event.burn_segment_time(&model));
    let burn = burn_snapshot.burn_starting_now(vessel);
    assert_float_equal(burn.total_dv(), 0.0, 0.2);


    let turn_snapshot = model.snapshot_at(burn_event_time);
    let turn = turn_snapshot.turn_starting_now(vessel);

    
    println!("mass after = {mass_after} mass before = {mass_before}");
    println!("position after = {position_after:?} position before = {position_before:?}");
    println!("velocity after = {velocity_after:?} velocity before = {velocity_before:?}");
    println!("rotation after = {rotation_after:?} rotation before = {rotation_before:?}");
    
    assert_float_equal(rotation_after, rotation_before, 1.0e-5);
    assert_float_equal(mass_after, mass_before, 1.0e-5);
    assert_dvec_equal(position_after, position_before, 1.0);
    assert_dvec_equal(velocity_after, velocity_before, 1.0e-5);
}
