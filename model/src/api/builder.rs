use ecolor::Color32;
use nalgebra_glm::{DVec2, vec2};

use crate::components::name_component::NameComponent;
use crate::components::orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType};
use crate::components::orbitable_component::atmosphere::Atmosphere;
use crate::components::path_component::orbit::Orbit;
use crate::components::path_component::orbit::orbit_direction::OrbitDirection;
use crate::components::path_component::PathComponent;
use crate::components::path_component::segment::Segment;
use crate::components::vessel_component::VesselComponent;
use crate::Model;
use crate::storage::entity_allocator::Entity;
use crate::storage::entity_builder::EntityBuilder;

#[derive(Debug, Clone)]
pub enum OrbitBuilder {
    Circular { parent: Entity, distance: f64, angle: f64, direction: OrbitDirection },
    Freeform { parent: Entity, distance: f64, angle: f64, direction: OrbitDirection, speed: f64 }
}

impl OrbitBuilder {
    pub fn build(&self, model: &Model, mass: f64) -> Orbit {
        match self {
            OrbitBuilder::Circular { parent, distance, angle, direction } => {
                let parent_mass = model.mass(*parent);
                let position = *distance * vec2(f64::cos(*angle), f64::sin(*angle));
                Orbit::circle(*parent, mass, parent_mass, position, model.time, *direction)
            }
            OrbitBuilder::Freeform { parent, distance, angle, direction, speed } => {
                let parent_mass = model.mass(*parent);
                let position = *distance * vec2(f64::cos(*angle), f64::sin(*angle));
                let mut velocity = *speed * vec2(f64::sin(*angle), -f64::cos(*angle));
                if let OrbitDirection::AntiClockwise = direction {
                    velocity *= -1.0;
                }
                Orbit::new(*parent, mass, parent_mass, position, velocity, model.time)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct VesselBuilder {
    pub name: &'static str,
    pub vessel_component: VesselComponent,
    pub orbit_builder: OrbitBuilder,
}

impl VesselBuilder {
    pub fn build(self, model: &mut Model) -> Entity {
        let orbit = self.orbit_builder.build(model, self.vessel_component.mass());
        model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new(self.name.to_string()))
            .with_vessel_component(self.vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)))
    }
}

#[derive(Debug, Clone)]
pub enum OrbitablePhysicsBuilder {
    Stationary(DVec2),
    Orbit(OrbitBuilder),
}

impl OrbitablePhysicsBuilder {
    pub fn build(&self, model: &Model, mass: f64) -> OrbitableComponentPhysics {
        match self {
            OrbitablePhysicsBuilder::Stationary(position) => OrbitableComponentPhysics::Stationary(*position),
            OrbitablePhysicsBuilder::Orbit(orbit_builder) => OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit_builder.build(model, mass))),
        }
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

#[derive(Debug, Clone)]
pub struct AtmosphereBuilder {
    pub color: Color32,
    pub density: f64,
    pub height: f64,
    pub falloff: f64,
}

impl AtmosphereBuilder {
    pub fn build(self) -> Atmosphere {
        Atmosphere::new(self.color, self.density, self.height, self.falloff)
    }
    
    pub fn build_some(self) -> Option<Atmosphere> {
        Some(self.build())
    }
}