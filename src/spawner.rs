use rand::{rngs::ThreadRng, Rng};

pub struct Spawner {
    rng: ThreadRng,
    min_interval: u32,
    eta: u32,
}

impl Spawner {
    pub fn new(min_interval: u32) -> Spawner {
        Spawner {
            rng: rand::thread_rng(),
            min_interval: min_interval + 1,
            eta: min_interval + 1,
        }
    }

    pub fn should_spawn(&mut self) -> bool {
        if self.eta == 0 {
            let interval = self.min_interval + self.rng.gen_range(0..100);
            self.eta = interval;
            return true;
        } else {
            self.eta -= 1;
            return false;
        }
    }
}
