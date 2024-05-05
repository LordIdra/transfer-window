use nalgebra_glm::vec2;

use crate::{components::{name_component::NameComponent, path_component::{orbit::Orbit, PathComponent}, vessel_component::{system_slot::SlotLocation, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

const EXPEL_VELOCITY: f64 = 10.0;
const TIME_BEFORE_BURN_START: f64 = 5.0;

impl Model {
    pub fn spawn_torpedo(&mut self, fire_from: Entity, slot_location: SlotLocation, target: Entity) {
        let parent = self.path_component(fire_from).current_segment().parent();
        let mut vessel_component = VesselComponent::new(VesselClass::Torpedo);
        // Subtract 0.1 because otherwise floating point errors might mean the burn is impossible
        let max_dv = vessel_component.max_dv().unwrap() - 1.0e-5;
        vessel_component.set_target(Some(target));
        let velocity = self.velocity(fire_from);
        let extra_velocity = vec2(-velocity.y, velocity.x).normalize() * EXPEL_VELOCITY;
        let orbit = Orbit::new(parent, vessel_component.mass(), self.mass(parent), self.position(fire_from), velocity + extra_velocity, self.time);
        let entity = self.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("Torpedo".to_string()))
            .with_vessel_component(vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)));
        self.recompute_trajectory(entity);
        let burn_time = self.time + TIME_BEFORE_BURN_START;
        self.create_burn(entity, burn_time);
        self.adjust_burn(entity, burn_time, vec2(0.0, max_dv))
    }
}