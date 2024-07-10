use super::View;

impl View {
    pub(crate) fn update_animation(&mut self, dt: f64) {
        for objective in &mut self.objectives {
            objective.update(dt);
        }
        self.objectives = self.objectives.iter().filter(|objective| !objective.finished()).cloned().collect();
    }
}