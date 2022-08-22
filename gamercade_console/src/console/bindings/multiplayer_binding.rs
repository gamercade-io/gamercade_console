use crate::api::{MultiplayerApi, MultiplayerApiBinding};
use paste::paste;
use wasmtime::{Caller, Linker};

use crate::console::Contexts;

macro_rules! derive_multiplayer_api_binding {
    ($($ident:ident ($($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl MultiplayerApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |caller: Caller<'_, Contexts>, $($name: $args,)*| {
                                caller.data().multiplayer_context.$ident($($name as $args,)*)
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_multiplayer_api_binding! {
    num_players(),
    is_local_player(player_id: i32),
    is_remote_player(player_id: i32),
}
