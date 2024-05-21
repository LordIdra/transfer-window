use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, PathComponent}, vessel_component::{timeline::{enable_guidance::EnableGuidanceEvent, TimelineEvent, TimelineEventType}, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_almost_static_target() {
    let mut model = Model::default();

    let parent_mass = 1.0e8;
    let orbitable_component = OrbitableComponent::new(parent_mass, 1.0e3, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)));
    let parent = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Parent".to_owned()))
        .with_orbitable_component(orbitable_component));
    
    let vessel_component = VesselComponent::new(VesselClass::Light);
    let orbit = Orbit::circle(parent, vessel_component.mass(), parent_mass, vec2(1.0e6, 0.0), 0.0, OrbitDirection::AntiClockwise);
    let target = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Target".to_owned()))
        .with_vessel_component(vessel_component)
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    let mut vessel_component = VesselComponent::new(VesselClass::Torpedo);
    vessel_component.set_target(Some(target));
    let orbit = Orbit::new(parent, vessel_component.mass(), parent_mass, vec2(1.0e6, -1.0e4), vec2(-10.0, 100.0), 0.0);
    let torpedo = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Torpedo".to_owned()))
        .with_vessel_component(vessel_component)
        .with_path_component(PathComponent::new_with_orbit(orbit)));

    model.recompute_trajectory(target);
    assert_eq!(model.path_component(target).future_segments().len(), 1);
    model.recompute_trajectory(torpedo);
    assert!(model.can_torpedo_enable_guidance(torpedo));
    let VesselClass::Torpedo = model.vessel_component_mut(torpedo).class_mut() else {
        unreachable!();
    };
    let event = TimelineEvent::new(0.0, TimelineEventType::EnableGuidance(EnableGuidanceEvent::new(&mut model, torpedo, 0.0)));
    model.add_event(torpedo, event);

    let time_to_step = 200.0;
    // let time_step = 0.0167; // about 1 frame
    let time_step = 1.0;
    while model.time() < time_to_step {
        model.update(time_step);
    }
    
    let intercept_time = model.find_next_closest_approach(torpedo, target, time_to_step).unwrap();
    dbg!(intercept_time, model.distance_at_time(torpedo, target, intercept_time));
    assert!(false);
}
