use log::trace;

use crate::model::{time::TimeStep, Model};

impl Model {
    pub(crate) fn update_warp(&mut self, dt: f64) {
        // Weird double if needed because of borrow checker
        let warp_finished = if let Some(warp) = &self.warp() {
            self.time() >= warp.end_time()
        } else {
            return;
        };
        
        if warp_finished {
            trace!("Warp finished");
            self.cancel_warp();
            self.set_time_step(TimeStep::Level { level: 1, paused: false });
        }

        if let Some(warp) = &self.warp() {
            let mut speed = warp.compute_warp_speed(self.time());
            let final_time = self.time() + speed * dt;
            if final_time > warp.end_time() {
                // Oh no, we're about to overshoot
                // Calculate required warp speed to perfectly land at target point
                // Add small amount so next frame actually counts this as 'finished'
                let overshot_time = warp.end_time() - self.time();
                speed = overshot_time / dt + 1.0e-3;
                trace!("Compensating for warp overshoot of {overshot_time}");
            }
            self.set_time_step(TimeStep::Warp { speed, paused: false });
        }
    }
}
