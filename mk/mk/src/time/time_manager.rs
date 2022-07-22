use std::time::Instant;

#[derive(Debug)]
pub struct TimeManager {
    time: Instant,
    dt: f32,
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            time: Instant::now(),
            dt: 0f32,
        }
    }

    pub fn time(&self) -> Instant {
        self.time
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.dt = (now - self.time).as_secs_f32();
        self.time = now;
    }
}
