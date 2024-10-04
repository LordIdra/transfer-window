use crate::{components::{path_component::{guidance::{builder::GuidanceBuilder, Guidance}, orbit::builder::OrbitBuilder, segment::Segment}, vessel_component::timeline::{intercept::InterceptEvent, TimelineEvent}}, storage::entity_allocator::Entity, Model};

impl Model {
    fn add_guidance(&mut self, entity: Entity, parent: Entity, guidance: Guidance) {
        let end_point = guidance.end_point().clone();
        let will_intercept = guidance.will_intercept();
        let target = guidance.target();
        self.path_component_mut(entity).add_segment(Segment::Guidance(guidance));

        if will_intercept {
            let intercept_time = end_point.time();
            let event = TimelineEvent::Intercept(InterceptEvent::new(self, entity, target, intercept_time));
            self.vessel_component_mut(entity)
                .timeline_mut()
                .add(event);
        }
        
        let parent_mass = self.mass(parent);
        let orbit = OrbitBuilder {
            parent,
            mass: end_point.mass(),
            parent_mass,
            rotation: end_point.rotation(),
            position: end_point.position(),
            velocity: end_point.velocity(),
            time: end_point.time(),
        }.build();

        self.path_component_mut(entity).add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn create_guidance(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Create guidance");
        assert!(self.vessel_component_mut(entity).class().is_torpedo());

        self.path_component_mut(entity).remove_segments_after(time);

        let last_segment = self.path_component(entity).end_segment();
        let parent = last_segment.parent();
        let guidance = GuidanceBuilder {
            parent,
            parent_mass: self.mass(parent),
            target: self.vessel_component(entity).target().expect("Cannot enable guidance on torpedo without a target"),
            faction: self.vessel_component(entity).faction(),
            engine: self.vessel_component(entity).engine().unwrap().clone(),
            mass: self.mass_at_time(entity, time, None),
            fuel_kg: self.fuel_kg_at_time(entity, time),
            time,
            rotation: last_segment.end_rotation(),
            position: last_segment.end_position(),
            velocity: last_segment.end_velocity(),
        }.build(self);

        // let guidance = Guidance::new(self, parent, target, faction, parent_mass, time, &rocket_equation_function, start_rotation, start_position, start_velocity);
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn recalculate_current_guidance(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recalculate current guidance");
        assert!(self.path_component(entity).current_segment().is_guidance());
        assert!(self.path_component(entity).current_segment().as_guidance().unwrap().will_intercept());

        let guidance = self.path_component(entity)
            .current_segment()
            .as_guidance()
            .unwrap();

        let parent = guidance.parent();
        let guidance = GuidanceBuilder {
            parent,
            parent_mass: self.mass(parent),
            target: guidance.target(),
            faction: self.vessel_component(entity).faction(),
            engine: self.vessel_component(entity).engine().unwrap().clone(),
            mass: self.mass_at_time(entity, self.time, None),
            fuel_kg: self.fuel_kg_at_time(entity, self.time),
            time: self.time,
            rotation: guidance.current_point().rotation(),
            position: guidance.current_point().position(),
            velocity: guidance.current_point().velocity(),
        }.build(self);

        self.path_component_mut(entity).clear_future_segments();
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn delete_guidance(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }
}
