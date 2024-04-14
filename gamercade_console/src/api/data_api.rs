pub trait DataApi {
    fn height(&self) -> i32;
    fn width(&self) -> i32;
    fn dimensions(&self) -> (i32, i32);
    fn fps(&self) -> i32;
    fn frame_time(&self) -> f32;

    fn sprite_sheet_count(&self) -> i32;
    fn palette_count(&self) -> i32;

    fn sprite_height(&self, sheet_index: i32) -> i32;
    fn sprite_width(&self, sheet_index: i32) -> i32;
    fn sprite_count(&self, sheet_index: i32) -> i32;
    fn sprite_dimensions(&self, sheet_index: i32) -> (i32, i32);

    fn bgm_length_secs(&self, bgm_index: i32) -> f32;
    fn bgm_length_frames(&self, bgm_index: i32) -> i32;
    fn sfx_length_secs(&self, sfx_index: i32) -> f32;
    fn sfx_length_frames(&self, sfx_index: i32) -> i32;
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
    bind_dimensions,
    bind_fps,
    bind_frame_time,
    bind_sprite_sheet_count,
    bind_palette_count,
    bind_sprite_height,
    bind_sprite_width,
    bind_sprite_dimensions,
    bind_sprite_count,
    bind_bgm_length_secs,
    bind_bgm_length_frames,
    bind_sfx_length_secs,
    bind_sfx_length_frames,
}
