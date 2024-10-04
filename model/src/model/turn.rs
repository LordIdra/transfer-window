use crate::{components::path_component::{orbit::builder::OrbitBuilder, segment::Segment, turn::{builder::TurnBuilder, Turn}}, storage::entity_allocator::Entity, Model};

impl Model {
    pub(crate) fn create_turn(&mut self, entity: Entity, time: f64) {
        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).end_segment();
        let parent = last_segment.parent();
        let parent_mass = self.mass(parent);
        let dry_mass = self.vessel_component(entity).dry_mass();
        let fuel_kg = self.fuel_kg_at_time(entity, time);
        let rcs = self.vessel_component(entity)
            .rcs()
            .expect("Attempt to create a turn on a vessel without an engine")
            .clone();

        let turn = TurnBuilder {
            parent,
            parent_mass,
            dry_mass,
            fuel_kg,
            time,
            position: last_segment.end_position(),
            velocity: last_segment.end_velocity(),
            rotation: last_segment.end_rotation(),
            target_rotation: 0.0,
            rcs,
        }.build();

        let end_point = turn.end_point();

        let orbit = OrbitBuilder {
            parent,
            mass: end_point.mass(),
            parent_mass,
            rotation: end_point.rotation(),
            position: end_point.position(),
            velocity: end_point.velocity(),
            time: end_point.time()
        }.build();

        self.path_component_mut(entity).add_segment(Segment::Turn(turn));
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn delete_turn(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if there is no turn at the specified time
    pub(crate) fn adjust_turn(&mut self, entity: Entity, time: f64, amount: f64) {
        let mut turn = self.turn_starting_at_time(entity, time).clone();
        turn.adjust(amount);
        
        let parent = turn.parent();
        let orbit = OrbitBuilder {
            parent: turn.parent(),
            mass: turn.end_point().mass(),
            parent_mass: self.mass(parent),
            rotation: turn.end_point().rotation(),
            position: turn.end_point().position(),
            velocity: turn.end_point().velocity(),
            time: turn.end_point().time(),
        }.build();

        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(turn.start_point().time());
        path_component.add_segment(Segment::Turn(turn));
        path_component.add_segment(Segment::Orbit(orbit));

        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if the entity does not have a turn at the given time
    pub fn turn_starting_at_time(&self, entity: Entity, time: f64) -> &Turn {
        if let Some(path_component) = self.try_path_component(entity) {
            if let Some(Segment::Turn(turn)) = path_component.future_segment_starting_at_time(time) {
                return turn;
            }
        }

        panic!("There is no turn at the requested time")
    }
}
