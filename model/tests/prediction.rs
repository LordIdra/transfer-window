use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{orbit::builder::OrbitBuilder, segment::Segment, PathComponent}, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_builder::EntityBuilder, Model, SEGMENTS_TO_PREDICT};

/// This example was taken from insanity-1 (note insanity-1 case may have changed from time of writing)
#[test]
fn test_prediction() {
    let mut model = Model::default();
    
    let earth_mass = 5.972e24;
    let sun_mass = 1.989e30;
    let moon_mass = 7.348e22;
    let vessel_mass = 3.0e2;

    let orbitable_component = OrbitableComponent::new(sun_mass, 1.0, 10.0, 0.0, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None);
    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(orbitable_component));

    let orbit = OrbitBuilder {
        parent: sun,
        mass: earth_mass,
        parent_mass: sun_mass,
        rotation: 0.0,
        position: vec2(1.521e11, 0.0),
        velocity: vec2(0.0, -2.929e4),
        time: 0.0,
    }.build().with_end_at(1.0e10);
    
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, 10.0, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit)), None)));

    let orbit = OrbitBuilder {
        parent: earth,
        mass: moon_mass,
        parent_mass: earth_mass,
        rotation: 0.0,
        position: vec2(0.0, 0.1055e9),
        velocity: vec2(1.870e3, 0.0),
        time: 0.0,
    }.build().with_end_at(1.0e10);

    let _moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(moon_mass, 1.0, 10.0, 0.0, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit)), None)));

    // Do not end this orbit because we are expecting to do trajectory prediction
    let orbit = OrbitBuilder {
        parent: earth,
        mass: vessel_mass,
        parent_mass: earth_mass,
        rotation: 0.0,
        position: vec2(0.01041e9, 0.0),
        velocity: vec2(0.0, 8.250e3),
        time: 0.0,
    }.build();

    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Frigate1, Faction::Player))
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    model.recompute_trajectory(vessel);

    let segments = model.path_component(vessel).future_segments();

    // + 1 to account for the last segment which will have a time of 0
    assert_eq!(segments.len(), SEGMENTS_TO_PREDICT + 1);
    
    let encounter_times = [1_452_880.599_685_907_4, 1_453_237.873_270_511_6, 1_756_031.469_329_595_6, 1_759_789.876_391_887_7];

    for i in 0..segments.len()-1 {
        assert_eq!(segments[i].end_time(), segments[i+1].start_time());
    }

    for i in 0..encounter_times.len() {
        let actual = segments[i].end_time();
        println!("Expected={} Actual={}", encounter_times[i], actual);
        assert!((actual - encounter_times[i]).abs() / encounter_times[i] < 1.0e-2);
    }
}

