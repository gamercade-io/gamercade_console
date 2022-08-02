use crate::raw;

pub fn set_seed(seed: i32) {
    unsafe { raw::set_seed(seed) }
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    unsafe { raw::random_int_range(min, max) }
}

pub fn random_float() -> f32 {
    unsafe { raw::random_float() }
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    unsafe { raw::random_float_range(min, max) }
}
