use time;

pub struct FrameTime {
    frame_start: f64,
    frame_s: f64,
    total_s: f64,
}

impl FrameTime {
    pub fn new() -> FrameTime {
        FrameTime {
            frame_start: time::precise_time_s(),
            frame_s: 0.0,
            total_s: 0.0,
        }
    }

    pub fn update(&mut self) {
        let frame_end = time::precise_time_s();
        self.frame_s = frame_end - self.frame_start;
        self.total_s += self.frame_s;

        self.frame_start = frame_end;
    }

    pub fn elapsed_s(&self) -> f32 {
        self.frame_s as f32
    }

    pub fn total_s(&self) -> f32 {
        self.total_s as f32
    }
}
