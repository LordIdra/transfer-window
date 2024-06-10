use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, orbit::Orbit, segment::Segment, PathComponent}, vessel_component::{Faction, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model, SEGMENTS_TO_PREDICT};

/// This example was taken from insanity-1 (note insanity-1 case may have changed from time of writing)
#[test]
fn test_prediction() {
    let mut model = Model::default();
    
    let earth_mass = 5.972e24;
    let sun_mass = 1.989e30;
    let moon_mass = 7.348e22;
    let vessel_mass = 3.0e2;

    let orbitable_component = OrbitableComponent::new(sun_mass, 1.0, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)));
    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(orbitable_component));

    let orbit = Orbit::new(sun, earth_mass, sun_mass, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 0.0)
        .with_end_at(1.0e10);
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, OrbitableType::Planet, OrbitableComponentPhysics::Orbit(orbit))));

    let orbit = Orbit::new(earth, moon_mass, earth_mass, vec2(0.0, 0.1055e9), vec2(1.870e3, 0.0), 0.0)
        .with_end_at(1.0e10);
    let _moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(moon_mass, 1.0, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(orbit))));

    // Do not end this orbit because we are expecting to do trajectory prediction
    let orbit = Orbit::new(earth, vessel_mass, earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light, Faction::Player))
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    model.recompute_trajectory(vessel);

    let segments = model.path_component(vessel).future_segments();

    // + 1 to account for the last segment which will have a time of 0
    assert_eq!(segments.len(), SEGMENTS_TO_PREDICT + 1);
    
    let encounter_times = vec![1452880.5996859074, 1453237.8732705116, 1756031.4693295956, 1759789.8763918877];

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
        .with_orbitable_component(OrbitableComponent::new(sun_mass, 1.0, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    let orbit = Orbit::new(sun, earth_mass, sun_mass, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 0.0)
        .with_end_at(1.0e10);
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0, OrbitableType::Planet, OrbitableComponentPhysics::Orbit(orbit))));

    let orbit = Orbit::new(earth, moon_mass, earth_mass, vec2(0.0, 0.1055e9), vec2(1.870e3, 0.0), 0.0)
        .with_end_at(1.0e10);
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(moon_mass, 1.0, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(orbit))));

    // Do not end this orbit because we are expecting to do trajectory prediction
    let vessel_start_position = vec2(0.01041e9, 0.0);
    let vessel_start_velocity = vec2(0.0, 7.250e3);
    let orbit = Orbit::new(earth, vessel_mass, earth_mass, vessel_start_position, vessel_start_velocity, 0.0);
    let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 10000.0, 0.0);
    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light, Faction::Player))
        .with_path_component(PathComponent::default()
            .with_segment(Segment::Orbit(orbit))));

        
    assert_eq!(model.path_component(vessel).future_segments().len(), 1);
        
    model.path_component_mut(vessel).final_segment_mut().as_orbit_mut().unwrap().end_at(0.0);
    let burn = Burn::new(earth, earth_mass, vessel_start_velocity.normalize(), vec2(1.0e3, 0.0), 0.0, rocket_equation_function, vessel_start_position, vessel_start_velocity);
    model.path_component_mut(vessel).add_segment(Segment::Burn(burn.clone()));

    let end_point = burn.end_point();
    let orbit = Orbit::new(earth, vessel_mass, earth_mass, end_point.position(), end_point.velocity(), end_point.time());
    model.path_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    println!("At end of burn, vessel position={:?} velocity={:?} time={} mass={}", end_point.position(), end_point.velocity(), end_point.time(), end_point.mass());
    println!("At end of burn, moon position={:?} velocity={:?}", model.position(moon), model.velocity(moon));
    println!("At end of burn, earth position={:?} velocity={:?}", model.position(earth), model.velocity(earth));
    
    model.recompute_trajectory(vessel);
    model.update(end_point.time() + 1.0e-3);

    let segments = model.path_component(vessel).future_segments();

    assert_eq!(segments.len(), SEGMENTS_TO_PREDICT + 1);

    let encounter_times = vec![1451640.0092875957, 1453650.030605793, 1756813.440374136, 1760025.6886267662];

    for i in 3..segments.len()-1 {
        assert_eq!(segments[i].end_time(), segments[i+1].start_time());
    }

    for i in 0..encounter_times.len() {
        let actual = segments[i].end_time();
        println!("Expected={} Actual={}", encounter_times[i], actual);
        assert!((actual - encounter_times[i]).abs() / encounter_times[i] < 1.0e-2);
    }
}