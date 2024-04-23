pub trait RandomApi {
    fn set_seed(&mut self, seed: i32);

    fn random_int_range(&mut self, min: i32, max: i32) -> i32;

    fn random_float(&mut self) -> f32;
    fn random_float_range(&mut self, min: f32, max: f32) -> f32;
}

macro_rules! derive_bind_random_api {
    ($($name:ident,)*) => {
        pub trait RandomApiBinding {
            $(fn $name(&mut self);)*

            fn bind_random_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

derive_bind_random_api! {
    bind_set_seed,
    bind_random_int_range,
    bind_random_float,
    bind_random_float_range,
}
