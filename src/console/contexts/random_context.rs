use nanorand::{Rng, SeedableRng, WyRand};

use crate::api::RandomApi;

// TODO: Handle local rngs?
#[derive(Clone)]
pub struct RandomContext {
    shared_rng: WyRand,
}

impl RandomContext {
    pub fn new(shared_seed: u64) -> Self {
        Self {
            shared_rng: WyRand::new_seed(shared_seed),
        }
    }
}

impl RandomApi for RandomContext {
    fn set_seed(&mut self, seed: i32) {
        self.shared_rng.reseed((seed as u64).to_ne_bytes());
    }

    fn random_int_range(&mut self, min: i32, max: i32) -> i32 {
        self.shared_rng.generate_range(min..max)
    }

    fn random_float(&mut self) -> f32 {
        self.shared_rng.generate()
    }

    fn random_float_range(&mut self, min: f32, max: f32) -> f32 {
        let range = max - min;
        let scale = self.shared_rng.generate::<f32>() * max;
        (scale * range) + min
    }
}
