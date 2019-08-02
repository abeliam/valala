use std::time::Instant;

pub struct Clock {
    pub initial_instant: Instant,
    pub last_instant: Instant,
    pub fps: u128,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            initial_instant: Instant::now(),
            last_instant: Instant::now(),
            fps: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.fps = 1_000_000_000 / (now - self.last_instant).as_nanos();
        self.last_instant = now;
    }
}
