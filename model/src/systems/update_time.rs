use crate::story_event::StoryEvent;
use crate::Model;

impl Model {
    pub(crate) fn update_time(&mut self, dt: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update time");
        self.time += dt * self.time_step.time_step();
        self.add_story_event(StoryEvent::NewTime(self.time));
    }
}
