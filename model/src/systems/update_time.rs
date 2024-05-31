use crate::Model;

impl Model {
    pub(crate) fn update_time(&mut self, dt: f64) {
        self.time += dt * self.time_step.time_step();
    }
}