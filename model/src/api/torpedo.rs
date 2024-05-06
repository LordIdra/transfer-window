use nalgebra_glm::{vec2, DVec2};

use crate::{components::{name_component::NameComponent, path_component::{orbit::Orbit, PathComponent}, vessel_component::{system_slot::SlotLocation, timeline::{fire_torpedo::FireTorpedoEvent, TimelineEvent, TimelineEventType}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};

impl Model {
    pub fn add_fire_torpedo_event(&mut self, fire_from: Entity, slot_location: SlotLocation, time: f64) {
        let velocity = self.velocity_at_time(fire_from, time);
        let event_type = TimelineEventType::FireTorpedo(FireTorpedoEvent::new(slot_location, velocity));
        self.try_vessel_component_mut(fire_from).unwrap().timeline_mut().add(TimelineEvent::new(time, event_type))  
    }

    /// # Panics
    /// Panics if there is no create torpedo event at the specified time
    pub fn adjust_fire_torpedo_event(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let fire_torpedo_event = self.fire_torpedo_event_at_time_mut(entity, time)
            .unwrap_or_else(|| panic!("Burn not found at time {time}"));
        fire_torpedo_event.adjust(amount);
        // let mass = fire_torpedo_event.end_point().mass();

        // let end_segment = path_component.last_segment();
        // let parent = end_segment.parent();
        // let position = end_segment.end_position();
        // let velocity = end_segment.end_velocity();
        // let parent_mass = self.mass(parent);

        // // Needs to be recalculated after we adjust the burn
        // let end_time = self.path_component_mut(entity)
        //     .future_segment_starting_at_time(time)
        //     .unwrap_or_else(|| panic!("Burn not found at time {time}"))
        //     .end_time();

        // let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, end_time);

        // self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        // self.recompute_trajectory(entity);
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