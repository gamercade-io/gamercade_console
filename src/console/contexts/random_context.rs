use fastrand::Rng;

use crate::api::RandomApi;

#[derive(Clone)]
pub struct RandomContext {
    shared_rng: Rng,
}

impl RandomContext {
    pub fn new(shared_seed: u64) -> Self {
        Self {
            shared_rng: Rng::with_seed(shared_seed),
        }
    }
}

impl RandomApi for RandomContext {
    fn set_seed(&self, seed: i32) {
        self.shared_rng.seed(seed as u64);
    }

    fn random_int_range(&self, min: i32, max: i32) -> i32 {
        self.shared_rng.i32(min..max)
    }

    fn random_float(&self) -> f32 {
        self.shared_rng.f32()
    }

    fn random_float_range(&self, min: f32, max: f32) -> f32 {
        let range = max - min;
        let scale = self.shared_rng.f32() * max;
        (scale * range) + min
    }
}
