use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}, vessel_component::{VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model, SEGMENTS_TO_PREDICT};

/// This example was taken from insanity-1 (note insanity-1 case may have changed from time of writing)
#[test]
fn test_prediction() {
    let mut model = Model::default();
    
    let earth_mass = 5.972e24;
    let sun_mass = 1.989e30;
    let moon_mass = 7.348e22;
    let vessel_mass = 3.0e2;

    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(OrbitableComponent::new(sun_mass, 1.0))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let orbit = Orbit::new(sun, earth_mass, sun_mass, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 0.0)
        .with_end_at(1.0e10);
    let trajectory = TrajectoryComponent::default()
        .with_segment(Segment::Orbit(orbit));
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0))
        .with_trajectory_component(trajectory));

    let orbit = Orbit::new(earth, moon_mass, earth_mass, vec2(0.0, 0.1055e9), vec2(1.870e3, 0.0), 0.0)
        .with_end_at(1.0e10);
    let trajectory = TrajectoryComponent::default()
        .with_segment(Segment::Orbit(orbit));
    let _moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(moon_mass, 1.0))
        .with_trajectory_component(trajectory));

    // Do not end this orbit because we are expecting to do trajectory prediction
    let orbit = Orbit::new(earth, vessel_mass, earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0);
    let trajectory = TrajectoryComponent::default()
        .with_segment(Segment::Orbit(orbit));
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_trajectory_component(trajectory));

    model.predict(vessel, 1.0e10, SEGMENTS_TO_PREDICT);

    let segments = model.get_trajectory_component(vessel).get_segments();

    // + 1 to account for the last segment which will have a time of 0
    assert_eq!(segments.len(), SEGMENTS_TO_PREDICT + 1);
    
    let encounter_times = vec![1452880.5996859074, 1453237.8732705116, 1756031.4693295956, 1759789.8763918877];

    for i in 0..segments.len()-1 {
        assert_eq!(segments[i].as_ref().unwrap().get_end_time(), segments[i+1].as_ref().unwrap().get_start_time());
    }

    for i in 0..encounter_times.len() {
        let actual = segments[i].as_ref().unwrap().get_end_time();
        println!("Expected={} Actual={}", encounter_times[i], actual);
        assert!((actual - encounter_times[i]).abs() / encounter_times[i] < 1.0e-3);
    }
}