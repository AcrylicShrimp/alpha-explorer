use std::time::Instant;

#[derive(Debug)]
pub struct TimeManager {
    begin: Instant,
    time: Instant,
    dt: f64,
}

impl TimeManager {
    pub fn new() -> TimeManager {
        let now = Instant::now();
        TimeManager {
            begin: now,
            time: now,
            dt: 0f64,
        }
    }

    pub fn time(&self) -> f32 {
        (self.time - self.begin).as_secs_f32()
    }

    pub fn time_f64(&self) -> f64 {
        (self.time - self.begin).as_secs_f64()
    }

    pub fn dt(&self) -> f32 {
        self.dt as f32
    }

    pub fn dt_f64(&self) -> f64 {
        self.dt
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.dt = (now - self.time).as_secs_f64();
        self.time = now;
    }
}
