use log::trace;

use crate::{components::{path_component::guidance::{will_intercept, Guidance}, vessel_component::timeline::{enable_guidance::EnableGuidanceEvent, TimelineEvent}, ComponentType}, Model};

/// True if would have intercepted last frame but now will not intercept
fn should_recalculate(model: &Model, guidance: &Guidance) -> bool {
    let end_distance = (guidance.end_point().position() - model.position_at_time(guidance.target(), guidance.end_point().time())).magnitude();
    guidance.will_intercept() && !will_intercept(end_distance)
}

impl Model {
    /// Handles recalculation of guidance segments which had an intercept,
    /// but do not any longer
    pub(crate) fn update_guidance(&mut self) {
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            if let Some(guidance) = self.path_component(entity).final_guidance() {
                if should_recalculate(self, guidance) {
                    assert!(self.vessel_component(entity).timeline().last_event().unwrap().is_intercept());
                    self.cancel_last_event(entity);

                    let on_guidance_segment_to_recalculate = self.path_component(entity).final_segment().start_time() == self.path_component(entity).current_segment().start_time();
                    
                    if on_guidance_segment_to_recalculate {
                        trace!("Recalculating guidance for current segment");
                        self.recalculate_current_guidance(entity);
                    } else {
                        trace!("Recalculating guidance for future segment");
                        let event = self.vessel_component(entity).timeline().last_event().unwrap();
                        assert!(event.is_enable_guidance());
                        let time = event.time();
                        self.cancel_last_event(entity);
                        let event = TimelineEvent::EnableGuidance(EnableGuidanceEvent::new(self, entity, time));
                        self.vessel_component_mut(entity).timeline_mut().add(event);
                    }
                }
            }
        }
    }
}