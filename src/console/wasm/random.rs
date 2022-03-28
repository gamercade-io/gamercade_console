use crate::{
    api::{RandomApi, RandomApiBinding},
    console::RandomContext,
};
use paste::paste;
use wasmer::Function;

use super::WasmConsoleBuilder;

macro_rules! derive_random_api_binding {
    ($($ident:ident,)*) => {
        paste! {
            impl RandomApiBinding for WasmConsoleBuilder<'_> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.imports.push((
                            stringify!($ident),
                            Function::new_native_with_env(
                                self.store,
                                self.random_context.clone(),
                                RandomContext::$ident)
                            ));
                    }
                )*
            }
        }
    }
}

derive_random_api_binding! {
    set_seed,
    get_seed,
    random_int,
    random_float,
    random_float_range,
    local_random_int,
    local_random_float_range,
    local_random_float,
}
