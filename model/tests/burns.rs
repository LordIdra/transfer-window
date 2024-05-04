use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}, vessel_component::{system_slot::{engine::{Engine, EngineType}, Slot, SlotLocation}, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};

#[test]
fn test_burn_without_engine_or_fuel_tank() {
    let mut model = Model::default();

    let earth_mass = 5.972e24;
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(earth_mass, 1.0))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    let vessel = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Vessel".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_trajectory_component(TrajectoryComponent::default()));
    let orbit = Orbit::new(earth, VesselClass::Light.get_mass(), earth_mass, vec2(0.01041e9, 0.0), vec2(0.0, 8.250e3), 0.0);
    model.get_trajectory_component_mut(vessel).add_segment(Segment::Orbit(orbit));

    let vessel_component = model.get_vessel_component_mut(vessel).get_slots_mut();
    let engine = Slot::Engine(Some(Engine::new(EngineType::Efficient)));
    *vessel_component.get_mut(SlotLocation::Back) = engine;

    // TODO better burn API then finish this test
}