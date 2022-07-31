// TODO: Write this
pub trait MultiplayerApi {
    fn num_players(&self) -> i32;
    fn is_local_player(&self, player_id: i32) -> i32;
    fn is_remote_player(&self, player_id: i32) -> i32;
}

macro_rules! derive_bind_multiplayer_api {
    ($($name:ident,)*) => {
        pub trait MultiplayerApiBinding {
            $(fn $name(&mut self);)*

            fn bind_multiplayer_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

derive_bind_multiplayer_api! {
    bind_num_players,
    bind_is_local_player,
    bind_is_remote_player,
}
