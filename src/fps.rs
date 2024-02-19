use bounded_vec_deque::BoundedVecDeque;
use std::time::{Instant, SystemTime};

pub struct FPSMonitor {
    frames: BoundedVecDeque<f32>,
    last_instant: Option<Instant>,
    frame_count: usize,
}

impl FPSMonitor {
    pub fn new() -> Self {
        Self {
            frames: BoundedVecDeque::new(50),
            last_instant: None,
            frame_count: 0,
        }
    }

    pub fn add_frame(&mut self, at: Instant) {
        if let Some(last) = self.last_instant {
            let elapsed = at.duration_since(last);
            self.frames.push_back(elapsed.as_secs_f32());
        }
        self.last_instant = Some(at);
        self.frame_count += 1;
    }

    pub fn log_fps(&mut self) {
        if self.frame_count > 20 {
            let mean_elapsed: f32 = self.frames.iter().sum::<f32>() / self.frames.len() as f32;
            let mean_fps = 1. / mean_elapsed;
            self.frame_count = 0;
            println!("FPS= {mean_fps}");
        }
    }
}
