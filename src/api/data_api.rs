pub trait DataApi {
    fn height(&self) -> i32;
    fn width(&self) -> i32;
    fn fps(&self) -> i32;
    fn frame_time(&self) -> f32;
    fn num_players(&self) -> i32;

    fn sprite_sheet_count(&self) -> i32;
    fn palette_count(&self) -> i32;

    fn sprite_height(&self, sheet_index: i32) -> i32;
    fn sprite_width(&self, sheet_index: i32) -> i32;
    fn sprite_count(&self, sheet_index: i32) -> i32;
}

macro_rules! derive_bind_data_api {
    ($($name:ident,)*) => {
        pub trait DataApiBinding {
            $(fn $name(&mut self);)*

            fn bind_data_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

derive_bind_data_api! {
    bind_height,
    bind_width,
    bind_fps,
    bind_frame_time,
    bind_sprite_sheet_count,
    bind_palette_count,
    bind_sprite_height,
    bind_sprite_width,
    bind_sprite_count,
    bind_num_players,
}
