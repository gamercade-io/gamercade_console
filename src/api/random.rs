use crate::raw;

/// Sets the random number generator to the desired seed.
pub fn set_seed(seed: i32) {
    unsafe { raw::set_seed(seed) }
}

/// Gets a random number from min, up to max. Max is non-inclusive.
/// For example, random_int_range(0, 10) will return any value
/// from 0 to 9, but not 10.
pub fn random_int_range(min: i32, max: i32) -> i32 {
    unsafe { raw::random_int_range(min, max) }
}

/// Gets a random float between 0.0 and 1.0.
pub fn random_float() -> f32 {
    unsafe { raw::random_float() }
}

/// Gets a random float between min and max.
pub fn random_float_range(min: f32, max: f32) -> f32 {
    unsafe { raw::random_float_range(min, max) }
}
