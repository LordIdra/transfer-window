use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{components::{name_component::NameComponent, path_component::{burn::rocket_equation_function::RocketEquationFunction, orbit::Orbit, PathComponent}, vessel_component::class::VesselClass}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

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
        let mut vessel_component = VesselClass::Torpedo.build(model.vessel_component(fire_from).faction());
        vessel_component.set_target(model.vessel_component(fire_from).target());

        let fire_from_orbit = model.orbit_at_time(fire_from, time, None);
        let point_at_time = fire_from_orbit.point_at_time(time);
        let parent_mass = model.mass(fire_from_orbit.parent());
        let rocket_equation_function = RocketEquationFunction::from_vessel_component(&vessel_component);
        let orbit = Orbit::new(
            fire_from_orbit.parent(), rocket_equation_function.mass(), parent_mass, 
            point_at_time.position(), point_at_time.velocity(), time);

        let ghost = model.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("Torpedo".to_string()))
            .with_path_component(PathComponent::new_with_orbit(orbit))
            .with_vessel_component(vessel_component));
        
        let burn_time = time + TIME_BEFORE_BURN_START;
        model.recompute_trajectory(ghost);
        model.create_burn(ghost, burn_time, rocket_equation_function);
        model.adjust_burn(ghost, burn_time, INITIAL_DV);

        Self { time, fire_from, ghost, burn_time }
    }

    /// # Panics
    /// Panics if the weapon slot requested does not in fact contain a weapon or is not a torpedo
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
        let cooldown = vessel_component.torpedo_launcher.as_ref().unwrap().type_().cooldown();
        if let Some(event) = vessel_component.timeline.last_fire_torpedo_event() {
            if event.time + cooldown > time {
                return false;
            }
        } else if model.time + vessel_component.torpedo_launcher.as_ref().unwrap().time_to_reload() > time {
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