use nalgebra_glm::DVec2;
use serde::{Deserialize, Serialize};

use crate::{components::{name_component::NameComponent, path_component::{burn::rocket_equation_function::RocketEquationFunction, orbit::Orbit, PathComponent}, vessel_component::{class::torpedo::Torpedo, system_slot::{SlotLocation, System}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

const TIME_BEFORE_BURN_START: f64 = 0.1;
const INITIAL_DV: DVec2 = DVec2::new(0.0, 1.0);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FireTorpedoEvent {
    fire_from: Entity,
    ghost: Entity,
    slot_location: SlotLocation,
    burn_time: f64,
}

impl FireTorpedoEvent {
    pub fn new(model: &mut Model, fire_from: Entity, time: f64, slot_location: SlotLocation) -> Self {
        let mut vessel_component = VesselComponent::new(VesselClass::Torpedo(Torpedo::default())).with_ghost();
        vessel_component.set_target(model.vessel_component(fire_from).target);

        let fire_from_orbit = model.orbit_at_time(fire_from, time);
        let point_at_time = fire_from_orbit.point_at_time(time);
        let parent_mass = model.mass_at_time(fire_from_orbit.parent(), time);
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

        Self { fire_from, ghost, slot_location, burn_time }
    }

    /// # Panics
    /// Panics if the weapon slot requested does not in fact contain a weapon or is not a torpedo
    pub fn execute(&self, model: &mut Model) {
        model.vessel_component_mut(self.ghost).set_ghost(false);
        let weapon = model.vessel_component_mut(self.fire_from)
            .slots_mut()
            .get_mut(self.slot_location)
            .as_weapon_mut()
            .expect("Weapon slot does not contain a weapon");
        weapon.type_mut().as_torpedo_mut().deplete();
    }

    pub fn cancel(&self, model: &mut Model) {
        model.deallocate(self.ghost);
    }

    pub fn adjust(&self, model: &mut Model, amount: DVec2) {
        model.adjust_burn(self.ghost(), self.burn_time(), amount);
    }

    pub fn ghost(&self) -> Entity {
        self.ghost
    }
    
    pub fn burn_time(&self) -> f64 {
        self.burn_time
    }

    pub fn slot_location(&self) -> SlotLocation {
        self.slot_location
    }
}