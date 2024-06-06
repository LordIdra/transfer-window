//! <https://github.com/emilk/egui/blob/78d95f430b09e3040d3f383fde8fea2ae1592afb/crates/egui_demo_app/src/frame_history.rs>

use eframe::emath::History;

const MAX_AGE: f32 = 1.0;

pub struct FrameHistory {
    frame_times: History<f32>
}

impl Default for FrameHistory {
    fn default() -> Self {
        let max_len = (MAX_AGE * 300.0).round() as usize;
        Self {
            frame_times: History::new(0..max_len, MAX_AGE),
        }
    }
}

impl FrameHistory {
    pub fn update(&mut self, now: f64, previous_frame_time: Option<f32>) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update frame history");
        let previous_frame_time = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time; // rewrite history now that we know
        }
        self.frame_times.add(now, previous_frame_time); // projected
    }

    pub fn fps(&self) -> usize {
        (1.0 / self.frame_times.mean_time_interval().unwrap_or_default()) as usize
    }
}