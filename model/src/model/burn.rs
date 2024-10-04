use nalgebra_glm::{vec2, DMat2, DVec2};

use crate::{components::path_component::{burn::{builder::BurnBuilder, Burn}, orbit::builder::OrbitBuilder, segment::Segment, turn::builder::TurnBuilder}, storage::entity_allocator::Entity, Model};

const MIN_DV_TO_ADJUST_BURN: f64 = 1.0e-2;

impl Model {
    pub(crate) fn create_burn(&mut self, entity: Entity, time: f64, delta_v: DVec2) {
        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).end_segment();
        let parent = last_segment.parent();
        let parent_mass = self.mass(parent);
        let mass_kg = last_segment.end_mass();
        let fuel_kg = self.fuel_kg_at_time(entity, time);
        let dry_mass = mass_kg - fuel_kg;
        let engine = self.vessel_component(entity)
            .engine()
            .expect("Attempt to create a burn on a vessel without an engine")
            .clone();
        let rcs = self.vessel_component(entity)
            .rcs()
            .expect("Attempt to create a burn on a vessel without an RCS")
            .clone();
        let tangent = last_segment.end_velocity().normalize();
        let absolute_delta_v = DMat2::new(
            tangent.x, -tangent.y, 
            tangent.y, tangent.x
        ) * delta_v;
        let target_rotation = f64::atan2(absolute_delta_v.y, absolute_delta_v.x);

        let turn = TurnBuilder {
            parent,
            parent_mass,
            dry_mass,
            fuel_kg,
            time,
            position: last_segment.end_position(),
            velocity: last_segment.end_velocity(),
            rotation: last_segment.end_rotation(),
            target_rotation,
            rcs,
        }.build();

        let turn_end_point = turn.end_point();

        let burn = BurnBuilder {
            parent,
            parent_mass,
            mass: mass_kg,
            fuel_kg,
            engine,
            tangent: turn_end_point.velocity().normalize(),
            delta_v,
            time: turn_end_point.time(),
            position: turn_end_point.position(),
            velocity: turn_end_point.velocity(),
        }.build();

        let burn_end_point = burn.end_point();

        let orbit = OrbitBuilder {
            parent,
            mass: burn_end_point.mass(),
            parent_mass,
            rotation: burn.rotation(),
            position: burn_end_point.position(),
            velocity: burn_end_point.velocity(),
            time: burn_end_point.time()
        }.build();

        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn delete_burn(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if there is no burn at the specified time
    pub(crate) fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let mut burn = self.burn_starting_at_time(entity, time).clone();
        burn.adjust(amount);
        
        let parent = burn.parent();
        let orbit = OrbitBuilder {
            parent: burn.parent(),
            mass: burn.end_point().mass(),
            parent_mass: self.mass(parent),
            rotation: burn.rotation(),
            position: burn.end_point().position(),
            velocity: burn.end_point().velocity(),
            time: burn.end_point().time(),
        }.build();

        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(burn.start_point().time());
        path_component.add_segment(Segment::Burn(burn));
        path_component.add_segment(Segment::Orbit(orbit));

        self.recompute_trajectory(entity);
    }

    /// # Panics
    /// Panics if the entity does not have a burn at the given time
    pub fn burn_starting_at_time(&self, entity: Entity, time: f64) -> &Burn {
        if let Some(path_component) = self.try_path_component(entity) {
            if let Some(Segment::Burn(burn)) = path_component.future_segment_starting_at_time(time) {
                return burn;
            }
        }

        panic!("There is no burn at the requested time")
    }

    pub fn calculate_burn_dv(&self, entity: Entity, time: f64, change: DVec2) -> Option<DVec2> {
        let burn = self.burn_starting_at_time(entity, time);
        let new_dv = (burn.delta_v() + change).magnitude();
        if new_dv > burn.start_remaining_dv() {
            if burn.end_remaining_dv() < MIN_DV_TO_ADJUST_BURN {
                None
            } else {
                Some(change.normalize() * burn.end_remaining_dv() * 0.999)
            }
        } else {
            Some(change)
        }
    }
}
