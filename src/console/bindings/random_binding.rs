use crate::api::{RandomApi, RandomApiBinding};
use paste::paste;
use wasmtime::{Caller, Linker};

use crate::console::Contexts;

macro_rules! derive_random_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl RandomApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |mut caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data_mut().random_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_random_api_binding! {
    set_seed(seed: i32),
    random_int_range(min: i32, max: i32),
    random_float(),
    random_float_range(min: f32, max: f32),
}
