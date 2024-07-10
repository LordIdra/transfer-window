use log::trace;

use crate::components::path_component::guidance::{will_intercept, Guidance};
use crate::components::vessel_component::timeline::enable_guidance::EnableGuidanceEvent;
use crate::components::vessel_component::timeline::TimelineEvent;
use crate::components::ComponentType;
use crate::Model;

/// True if would have intercepted last frame but now will not intercept
fn should_recalculate(model: &Model, guidance: &Guidance) -> bool {
    let target = guidance.target();
    let time = guidance.end_point().time();
    let faction = guidance.faction();
    let end_distance = (guidance.end_point().position()
        - model.position_at_time(target, time, Some(faction)))
    .magnitude();
    guidance.will_intercept() && !will_intercept(end_distance)
}

impl Model {
    /// Handles recalculation of guidance segments which had an intercept, but
    /// do not any longer.
    pub(crate) fn update_guidance(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update guidance");
        for entity in self.entities(vec![
            ComponentType::VesselComponent,
            ComponentType::PathComponent,
        ]) {
            let Some(guidance) = self.path_component(entity).final_guidance() else {
                continue;
            };

            if self.docked(guidance.target()) {
                if self
                    .vessel_component(entity)
                    .timeline()
                    .last_event()
                    .is_some_and(|event| event.is_intercept())
                {
                    self.cancel_last_event(entity);
                }
                continue;
            }

            if !should_recalculate(self, guidance) {
                continue;
            }

            assert!(self.vessel_component(entity).timeline().last_event().unwrap().is_intercept());
            self.cancel_last_event(entity);

            let on_guidance_segment_to_recalculate = self.time
                >= self.path_component(entity).final_guidance().unwrap().start_point().time();
            if on_guidance_segment_to_recalculate {
                trace!("Recalculating guidance for current segment");
                self.recalculate_current_guidance(entity);
            } else {
                trace!("Recalculating guidance for future segment");
                let event = self.vessel_component(entity).timeline().last_event().unwrap();
                assert!(event.is_enable_guidance());
                let time = event.time();
                self.cancel_last_event(entity);
                let event =
                    TimelineEvent::EnableGuidance(EnableGuidanceEvent::new(self, entity, time));
                self.vessel_component_mut(entity).timeline_mut().add(event);
            }
        }
    }
}
