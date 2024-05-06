use nalgebra_glm::vec2;

use crate::{components::{name_component::NameComponent, path_component::{orbit::Orbit, PathComponent}, vessel_component::{system_slot::SlotLocation, timeline::{fire_torpedo::FireTorpedoEvent, TimelineEvent, TimelineEventType}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

impl Model {
    pub fn add_fire_torpedo_event(&mut self, fire_from: Entity, slot_location: SlotLocation, time: f64) {
        let velocity = self.velocity_at_time(fire_from, time);
        let event_type = TimelineEventType::FireTorpedo(FireTorpedoEvent::new(slot_location, velocity));
        self.try_vessel_component_mut(fire_from).unwrap().add_timeline_event(TimelineEvent::new(time, event_type))  
    }

    fn spawn_torpedo(&mut self, fire_from: Entity, slot_location: SlotLocation, target: Entity) {
        let parent = self.path_component(fire_from).current_segment().parent();
        let mut vessel_component = VesselComponent::new(VesselClass::Torpedo);
        vessel_component.set_target(Some(target));
        let velocity = self.velocity(fire_from);
        let extra_velocity = vec2(-velocity.y, velocity.x).normalize();
        let orbit = Orbit::new(parent, vessel_component.mass(), self.mass(parent), self.position(fire_from), velocity + extra_velocity, self.time);
        let entity = self.allocate(EntityBuilder::default()
            .with_name_component(NameComponent::new("Torpedo".to_string()))
            .with_vessel_component(vessel_component)
            .with_path_component(PathComponent::new_with_orbit(orbit)));
        self.recompute_trajectory(entity);
    }
}