use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{burn::{builder::BurnBuilder, rocket_equation_function::RocketEquationFunction}, orbit::builder::OrbitBuilder, segment::Segment, PathComponent}, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_builder::EntityBuilder, Model, SEGMENTS_TO_PREDICT};

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

/// Encounter times computed using the insanity-1-for-burn-test case
#[test]
fn test_prediction_with_burn() {
    let mut model = Model::default();
    
    let earth_mass = 5.972e24;
    let sun_mass = 1.989e30;
    let moon_mass = 7.348e22;
    let vessel_mass = 3.0e2;

    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(OrbitableComponent::new(sun_mass, 1.0, 10.0, 0.0, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)), None)));

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

    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(moon_mass, 1.0, 10.0, 0.0, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit)), None)));

    // Do not end this orbit because we are expecting to do trajectory prediction
    let vessel_start_position = vec2(0.01041e9, 0.0);
    let vessel_start_velocity = vec2(0.0, 7.250e3);
    let orbit = OrbitBuilder {
        parent: earth,
        mass: vessel_mass,
        parent_mass: earth_mass,
        rotation: 0.0,
        position: vessel_start_position,
        velocity: vessel_start_velocity,
        time: 0.0,
    }.build().with_end_at(1.0e10);

    let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 10000.0, 0.0);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Frigate1, Faction::Player))
        .with_path_component(PathComponent::default()
            .with_segment(Segment::Orbit(orbit))));

        
    assert_eq!(model.path_component(vessel).future_segments().len(), 1);
        
    model.path_component_mut(vessel).final_segment_mut().as_orbit_mut().unwrap().end_at(0.0);
    
    let burn = BurnBuilder {
        parent: earth,
        parent_mass: earth_mass,
        rocket_equation_function,
        tangent: vessel_start_velocity.normalize(),
        delta_v: vec2(1.0e3, 0.0),
        time: 0.0,
        position: vessel_start_position,
        velocity: vessel_start_velocity,
    }.build();

    model.path_component_mut(vessel).add_segment(Segment::Burn(burn.clone()));

    let end_point = burn.end_point();

    let orbit = OrbitBuilder {
        parent: earth,
        mass: vessel_mass,
        parent_mass: earth_mass,
        rotation: burn.rotation(),
        position: end_point.position(),
        velocity: end_point.velocity(),
        time: end_point.time(),
    }.build();

    model.path_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    println!("At end of burn, vessel position={:?} velocity={:?} time={} mass={}", end_point.position(), end_point.velocity(), end_point.time(), end_point.mass());
    println!("At end of burn, moon position={:?} velocity={:?}", model.position(moon), model.velocity(moon));
    println!("At end of burn, earth position={:?} velocity={:?}", model.position(earth), model.velocity(earth));
    
    model.recompute_trajectory(vessel);
    model.update(end_point.time() + 1.0e-3);

    let segments = model.path_component(vessel).future_segments();

    assert_eq!(segments.len(), SEGMENTS_TO_PREDICT + 1);

    let encounter_times = [1_451_640.009_287_595_7, 1_453_650.030_605_793, 1_756_813.440_374_136, 1_760_025.688_626_766_2];

    for i in 3..segments.len()-1 {
        assert_eq!(segments[i].end_time(), segments[i+1].start_time());
    }

    for i in 0..encounter_times.len() {
        let actual = segments[i].end_time();
        println!("Expected={} Actual={}", encounter_times[i], actual);
        assert!((actual - encounter_times[i]).abs() / encounter_times[i] < 1.0e-2);
    }
}