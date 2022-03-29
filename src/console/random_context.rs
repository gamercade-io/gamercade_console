use crate::api::RandomApi;

// TODO: This
#[derive(Clone)]
pub struct RandomContext {}

impl RandomApi for RandomContext {
    fn set_seed(&self, seed: i32) {
        todo!()
    }

    fn get_seed(&self) -> i32 {
        todo!()
    }

    fn random_int(&self, min: i32, max: i32) -> i32 {
        todo!()
    }

    fn local_random_int(&self, min: i32, max: i32) -> i32 {
        todo!()
    }

    fn random_float(&self) -> f32 {
        todo!()
    }

    fn random_float_range(&self, min: f32, max: f32) -> f32 {
        todo!()
    }

    fn local_random_float(&self) {
        todo!()
    }

    fn local_random_float_range(&self, min: f32, max: f32) -> f32 {
        todo!()
    }
}
