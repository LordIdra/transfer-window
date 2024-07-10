use nalgebra_glm::{vec2, DVec2};

use crate::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::VesselComponent}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

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
        let entity =model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new(self.name.to_string()))
            .with_vessel_component(self.vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)));
        model.recompute_trajectory(entity);
        entity
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
}

impl OrbitableBuilder {
    pub fn build(self, model: &mut Model) -> Entity {
        let physics = self.physics.build(model, self.mass);
        let orbitable_component = OrbitableComponent::new(self.mass, self.radius, self.rotation_period, self.rotation_angle, self.type_, physics);
        model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new(self.name.to_string()))
            .with_orbitable_component(orbitable_component))
    }
}