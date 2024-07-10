use crate::components::path_component::guidance::Guidance;
use crate::components::path_component::orbit::Orbit;
use crate::components::path_component::segment::Segment;
use crate::components::vessel_component::timeline::intercept::InterceptEvent;
use crate::components::vessel_component::timeline::TimelineEvent;
use crate::storage::entity_allocator::Entity;
use crate::Model;

impl Model {
    fn add_guidance(&mut self, entity: Entity, parent: Entity, guidance: Guidance) {
        let end_point = guidance.end_point().clone();
        let will_intercept = guidance.will_intercept();
        let target = guidance.target();
        self.path_component_mut(entity)
            .add_segment(Segment::Guidance(guidance));

        if will_intercept {
            let intercept_time = end_point.time();
            let event =
                TimelineEvent::Intercept(InterceptEvent::new(self, entity, target, intercept_time));
            self.vessel_component_mut(entity).timeline_mut().add(event);
        }

        let parent_mass = self.mass(parent);
        let orbit = Orbit::new(
            parent,
            end_point.mass(),
            parent_mass,
            end_point.position(),
            end_point.velocity(),
            end_point.time(),
        );
        self.path_component_mut(entity)
            .add_segment(Segment::Orbit(orbit));
        self.recompute_trajectory(entity);
    }

    pub(crate) fn create_guidance(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Create guidance");
        assert!(self.vessel_component_mut(entity).class().is_torpedo());

        let target = self
            .vessel_component(entity)
            .target()
            .expect("Cannot enable guidance on torpedo without a target");
        let faction = self.vessel_component(entity).faction();
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);

        let last_segment = path_component.final_segment();
        let parent = last_segment.parent();
        let start_position = last_segment.end_position();
        let start_velocity = last_segment.end_velocity();
        let parent_mass = self.mass(parent);
        let rocket_equation_function = self.rocket_equation_function_at_end_of_trajectory(entity);
        let guidance = Guidance::new(
            self,
            parent,
            target,
            faction,
            parent_mass,
            time,
            &rocket_equation_function,
            start_position,
            start_velocity,
        );
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn recalculate_current_guidance(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Recalculate current guidance");
        assert!(self.path_component(entity).current_segment().is_guidance());
        assert!(
            self.path_component(entity)
                .current_segment()
                .as_guidance()
                .unwrap()
                .will_intercept()
        );

        let guidance = self
            .path_component(entity)
            .current_segment()
            .as_guidance()
            .unwrap();
        let parent = guidance.parent();
        let target = guidance.target();
        let faction = self.vessel_component(entity).faction();
        let start_position = guidance.current_point().position();
        let start_velocity = guidance.current_point().velocity();
        let parent_mass = self.mass(parent);
        let rocket_equation_function = guidance.rocket_equation_function_at_time(self.time);
        let guidance = Guidance::new(
            self,
            parent,
            target,
            faction,
            parent_mass,
            self.time,
            &rocket_equation_function,
            start_position,
            start_velocity,
        );
        self.path_component_mut(entity).clear_future_segments();
        self.add_guidance(entity, parent, guidance);
    }

    pub(crate) fn delete_guidance(&mut self, entity: Entity, time: f64) {
        let path_component = self.path_component_mut(entity);
        path_component.remove_segments_after(time);
        self.recompute_trajectory(entity);
    }
}
