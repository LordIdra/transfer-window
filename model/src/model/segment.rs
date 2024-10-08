use nalgebra_glm::{DMat2, DVec2};

use crate::{components::{path_component::{burn::builder::BurnBuilder, guidance::{builder::GuidanceBuilder, Guidance}, orbit::builder::OrbitBuilder, segment::Segment, turn::builder::TurnBuilder}, vessel_component::timeline::{intercept::InterceptEvent, TimelineEvent}}, storage::entity_allocator::Entity};

use super::{state_query::StateQuery, Model};

const MIN_DV_TO_ADJUST_BURN: f64 = 1.0e-2;

impl Model {
    pub fn delete_segment(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        assert!(path_component.future_segment_starting_at_time(time).is_some());
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }

    /// After we modify a trajectory, the end of that trajectory may be missing - for example we
    /// may insert a burn at some point, but then we'll need to recompute everything after that
    /// burn. This function inserts a corresponding orbit at the end of the known trajectory, then
    /// calls prediction for the rest.
    fn complete_trajectory(&mut self, entity: Entity) {
        let end_segment = self.path_component(entity).end_segment();
        let orbit = OrbitBuilder {
            parent: end_segment.parent(),
            mass: end_segment.end_mass(),
            parent_mass: self.snapshot_at(end_segment.end_time()).mass(end_segment.parent()),
            rotation: end_segment.end_rotation(),
            position: end_segment.end_position(),
            velocity: end_segment.end_velocity(),
            time: end_segment.end_time(),
        }.build();
        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    // Helper for guidance components which recomputes whether the guidance will make an intercept
    // and adds the appropriate intercept event if so
    fn add_guidance(&mut self, entity: Entity, guidance: Guidance) {
        if guidance.will_intercept() {
            let event = TimelineEvent::Intercept(InterceptEvent::new(self, entity, guidance.target(), guidance.end_point().time()));
            self.vessel_component_mut(entity)
                .timeline_mut()
                .add(event);
        }

        self.path_component_mut(entity).add_segment(Segment::Guidance(guidance));
        self.complete_trajectory(entity);
    }

    pub fn create_burn(&mut self, entity: Entity, time: f64, delta_v: DVec2) {
        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).end_segment();
        let tangent = last_segment.end_velocity().normalize();
        dbg!(tangent);
        let absolute_delta_v = DMat2::new(tangent.x, -tangent.y, tangent.y, tangent.x) * delta_v;
        let target_rotation = f64::atan2(absolute_delta_v.y, absolute_delta_v.x);
        dbg!(target_rotation);

        self.create_turn(entity, time, target_rotation);

        let snapshot = &self.snapshot_at(time);
        let turn = snapshot.turn_starting_now(entity);
        let turn_end_point = turn.end_point();
        let engine = self.vessel_component(entity)
            .engine()
            .expect("Attempt to create a burn on a vessel without an engine")
            .clone();

        let burn = BurnBuilder {
            parent: turn.parent(),
            parent_mass: self.snapshot_at(time).mass(turn.parent()),
            mass: turn_end_point.mass(),
            fuel_kg: self.snapshot_at(time).fuel_kg(entity),
            engine,
            tangent: turn_end_point.velocity().normalize(),
            delta_v,
            time: turn_end_point.time(),
            position: turn_end_point.position(),
            velocity: turn_end_point.velocity(),
        }.build();

        self.path_component_mut(entity).remove_segments_after(turn.end_point().time());
        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.complete_trajectory(entity);
    }

    pub fn create_guidance(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Create guidance");
        assert!(self.vessel_component_mut(entity).class().is_torpedo());

        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).end_segment();
        let guidance = GuidanceBuilder {
            parent: last_segment.parent(),
            parent_mass: self.mass(last_segment.parent()),
            target: self.vessel_component(entity).target().expect("Cannot enable guidance on torpedo without a target"),
            faction: self.vessel_component(entity).faction(),
            engine: self.vessel_component(entity).engine().unwrap().clone(),
            mass: self.snapshot_at(time).mass(entity),
            fuel_kg: self.snapshot_at(time).fuel_kg(entity),
            time,
            rotation: last_segment.end_rotation(),
            position: last_segment.end_position(),
            velocity: last_segment.end_velocity(),
        }.build(self);

        self.add_guidance(entity, guidance);
    }

    pub fn create_turn(&mut self, entity: Entity, time: f64, target_rotation: f64) {
        self.path_component_mut(entity).remove_segments_after(time);

        let rcs = self.vessel_component(entity)
            .rcs()
            .expect("Attempt to create a turn on a vessel without RCS")
            .clone();

        let end_segment = self.path_component(entity).end_segment();
        let turn = TurnBuilder {
            parent: end_segment.parent(),
            parent_mass: self.snapshot_at(end_segment.end_time()).mass(end_segment.parent()),
            dry_mass: self.vessel_component(entity).dry_mass(),
            fuel_kg: self.snapshot_at(end_segment.end_time()).fuel_kg(entity),
            time,
            position: end_segment.end_position(),
            velocity: end_segment.end_velocity(),
            rotation: end_segment.end_rotation(),
            target_rotation,
            rcs,
        }.build();

        self.path_component_mut(entity).add_segment(Segment::Turn(turn));
        self.complete_trajectory(entity);
    }

    pub fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        let snapshot = &self.snapshot_at(time);
        let mut burn = snapshot.burn_starting_now(entity).clone();
        burn.adjust(amount);
        self.path_component_mut(entity).remove_segments_after(burn.start_point().time());
        self.path_component_mut(entity).add_segment(Segment::Burn(burn));
        self.complete_trajectory(entity);
    }

    pub fn adjust_turn(&mut self, entity: Entity, time: f64, amount: f64) {
        let snapshot = &self.snapshot_at(time);
        let mut turn = snapshot.turn_starting_now(entity).clone();
        turn.adjust(amount);
        self.path_component_mut(entity).remove_segments_after(turn.start_point().time());
        self.path_component_mut(entity).add_segment(Segment::Turn(turn));
        self.complete_trajectory(entity);
    }

    pub fn recalculate_current_guidance(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recalculate current guidance");
        assert!(self.path_component(entity).current_segment().is_guidance());
        assert!(self.path_component(entity).current_segment().as_guidance().unwrap().will_intercept());

        let guidance = self.path_component(entity)
            .current_segment()
            .as_guidance()
            .unwrap();

        let guidance = GuidanceBuilder {
            parent: guidance.parent(),
            parent_mass: self.mass(guidance.parent()),
            target: guidance.target(),
            faction: self.vessel_component(entity).faction(),
            engine: self.vessel_component(entity).engine().unwrap().clone(),
            mass: self.mass(entity),
            fuel_kg: self.fuel_kg(entity),
            time: self.time,
            rotation: guidance.current_point().rotation(),
            position: guidance.current_point().position(),
            velocity: guidance.current_point().velocity(),
        }.build(self);

        self.path_component_mut(entity).clear_future_segments();
        self.add_guidance(entity, guidance);
    }

    pub fn burn_dv_after_adjustment(&self, entity: Entity, time: f64, change: DVec2) -> Option<DVec2> {
        let snapshot = &self.snapshot_at(time);
        let burn = snapshot.burn_starting_now(entity);
        let new_dv = (burn.delta_v() + change).magnitude();
        if new_dv > burn.start_remaining_dv() {
            if burn.end_dv() < MIN_DV_TO_ADJUST_BURN {
                None
            } else {
                Some(change.normalize() * burn.end_dv() * 0.999)
            }
        } else {
            Some(change)
        }
    }
}

