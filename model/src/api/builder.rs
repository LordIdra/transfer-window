use crate::{components::{name_component::NameComponent, orbitable_component::{atmosphere::Atmosphere, builder::OrbitablePhysicsBuilder, OrbitableComponent, OrbitableType}, path_component::{orbit::builder::InitialOrbitBuilder, PathComponent}, vessel_component::VesselComponent}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

#[derive(Debug, Clone)]
pub struct VesselBuilder {
    pub name: &'static str,
    pub vessel_component: VesselComponent,
    pub orbit_builder: InitialOrbitBuilder,
}

impl VesselBuilder {
    pub fn build(self, model: &mut Model) -> Entity {
        let orbit = self.orbit_builder.build(model, self.vessel_component.mass());
        let entity =model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new(self.name.to_string()))
            .with_vessel_component(self.vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)));
        model.recompute_trajectory(entity);
        entity
    }
}

#[derive(Debug, Clone)]
pub struct OrbitableBuilder {
    pub name: &'static str,
    pub mass: f64,
    pub radius: f64,
    pub rotation_period: f64,
    pub rotation_angle: f64,
    pub type_: OrbitableType,
    pub physics: OrbitablePhysicsBuilder,
    pub atmosphere: Option<Atmosphere>,
}

impl OrbitableBuilder {
    pub fn build(self, model: &mut Model) -> Entity {
        let physics = self.physics.build(model, self.mass);
        let orbitable_component = OrbitableComponent::new(
            self.mass,
            self.radius,
            self.rotation_period,
            self.rotation_angle,
            self.type_,
            physics,
            self.atmosphere
        );
        model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new(self.name.to_string()))
            .with_orbitable_component(orbitable_component))
    }
}