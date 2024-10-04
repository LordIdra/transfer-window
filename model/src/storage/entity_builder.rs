use crate::components::name_component::NameComponent;
use crate::components::orbitable_component::atmosphere::Atmosphere;
use crate::components::orbitable_component::builder::OrbitablePhysicsBuilder;
use crate::components::orbitable_component::{OrbitableComponent, OrbitableType};
use crate::components::path_component::orbit::builder::InitialOrbitBuilder;
use crate::components::path_component::PathComponent;
use crate::components::vessel_component::VesselComponent;
use crate::model::Model;

use super::entity_allocator::Entity;

#[derive(Debug, Default, Clone)]
pub struct EntityBuilder {
    pub name_component: Option<NameComponent>,
    pub orbitable_component: Option<OrbitableComponent>,
    pub path_component: Option<PathComponent>,
    pub vessel_component: Option<VesselComponent>,
}

impl EntityBuilder {
    pub fn with_name_component(mut self, component: NameComponent) -> Self {
        self.name_component = Some(component);
        self
    }

    pub fn with_orbitable_component(mut self, component: OrbitableComponent) -> Self {
        self.orbitable_component = Some(component);
        self
    }

    pub fn with_path_component(mut self, component: PathComponent) -> Self {
        self.path_component = Some(component);
        self
    }

    pub fn with_vessel_component(mut self, component: VesselComponent) -> Self {
        self.vessel_component = Some(component);
        self
    }

    pub fn build(self, model: &mut Model) -> Entity {
        model.allocate(self)
    }
}

#[derive(Debug, Clone)]
pub struct VesselBuilder {
    pub name: &'static str,
    pub vessel_component: VesselComponent,
    pub orbit_builder: InitialOrbitBuilder,
}

impl VesselBuilder {
    pub fn build(self, model: &mut Model) -> Entity {
        let orbit = self.orbit_builder.build(model, self.vessel_component.mass());
        let entity = model.allocate(EntityBuilder::default()
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
