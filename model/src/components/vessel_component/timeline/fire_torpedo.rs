use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{components::{name_component::NameComponent, path_component::{orbit::builder::OrbitBuilder, rocket_equation_function::RocketEquationFunction, PathComponent}, vessel_component::{class::VesselClass, VesselComponent}}, model::{state_query::StateQuery, Model}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}};

const TIME_BEFORE_BURN_START: f64 = 0.1;
const INITIAL_DV: DVec2 = DVec2::new(0.0, 1.0);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FireTorpedoEvent {
    time: f64,
    fire_from: Entity,
    ghost: Entity,
    burn_time: f64,
}

impl FireTorpedoEvent {
    pub fn new(model: &mut Model, fire_from: Entity, time: f64) -> Self {
        let mut vessel_component = VesselComponent::new(VesselClass::Torpedo, model.vessel_component(fire_from).faction());
        vessel_component.set_target(model.vessel_component(fire_from).target());

        let snapshot = &model.snapshot_at(time);
        let fire_from_orbit = snapshot.orbit(fire_from);
        let point_at_time = fire_from_orbit.point_at_time(time);
        let rocket_equation_function = RocketEquationFunction::fuel_from_vessel_component(&vessel_component);

        let orbit = OrbitBuilder {
            parent: fire_from_orbit.parent(),
            mass: rocket_equation_function.mass(),
            parent_mass: model.mass(fire_from_orbit.parent()),
            rotation: fire_from_orbit.rotation(),
            position: point_at_time.position(),
            velocity: point_at_time.velocity(),
            time,
        }.build();
        
        let ghost = model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("Torpedo".to_string()))
            .with_path_component(PathComponent::new_with_orbit(orbit))
            .with_vessel_component(vessel_component));
        
        let burn_time = time + TIME_BEFORE_BURN_START;
        model.recompute_trajectory(ghost);
        model.create_burn(ghost, burn_time, INITIAL_DV);

        Self { time, fire_from, ghost, burn_time }
    }

    /// # Panics
    /// Panics if the vessel does not have a torpedo launcher
    pub fn execute(&self, model: &mut Model) {
        model.vessel_component_mut(self.ghost).unset_ghost();
        model.vessel_component_mut(self.fire_from).decrement_torpedoes();
        model.vessel_component_mut(self.fire_from).torpedo_launcher.as_mut().unwrap().reset_time_to_reload();
    }

    pub fn cancel(&self, model: &mut Model) {
        model.deallocate(self.ghost);
    }

    pub fn adjust(&self, model: &mut Model, amount: DVec2) {
        model.adjust_burn(self.ghost(), self.burn_time(), amount);
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn is_blocking(&self) -> bool {
        true
    }
    
    pub fn can_remove(&self) -> bool {
        true
    }

    pub fn can_adjust(&self, model: &Model) -> bool {
        model.vessel_component(self.ghost).timeline().last_blocking_event().is_none()
    }

    pub fn can_create_ever(model: &Model, entity: Entity) -> bool {
        let vessel_component = &model.vessel_component(entity);
        vessel_component.has_torpedo_storage()
            && vessel_component.has_torpedo_launcher()
    }

    pub fn can_create(model: &Model, entity: Entity, time: f64) -> bool {
        let vessel_component = &model.vessel_component(entity);
        let cooldown = vessel_component.torpedo_launcher.as_ref().unwrap().cooldown();
        if let Some(event) = vessel_component.timeline.last_fire_torpedo_event() {
            if event.time + cooldown > time {
                return false;
            }
        } else if model.time() + vessel_component.torpedo_launcher.as_ref().unwrap().time_to_reload() > time {
            return false;
        }
        vessel_component.timeline().is_time_after_last_blocking_event(time)
            && vessel_component.final_torpedoes() != 0
    }

    pub fn ghost(&self) -> Entity {
        self.ghost
    }
    
    pub fn burn_time(&self) -> f64 {
        self.burn_time
    }
}
