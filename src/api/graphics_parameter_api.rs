pub trait GraphicsParameterApi {
    fn palette_index(&self, palette_index: i32) -> i32;
    fn sprite_sheet_index(&self, sprite_sheet_index: i32) -> i32;
    fn sprite_index(&self, sprite_index: i32) -> i32;
    fn color_index(&self, color_index: i32) -> i32;
    fn flip_x(&self, flip: i32) -> i32;
    fn flip_y(&self, flip: i32) -> i32;
}

macro_rules! derive_bind_graphics_parameter_api {
    ($($name:ident,)*) => {
        pub trait GraphicsParameterApiBinding {
            $(fn $name(&mut self);)*

            fn bind_graphics_parameter_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

derive_bind_graphics_parameter_api! {
    bind_palette_index,
    bind_sprite_sheet_index,
    bind_sprite_index,
    bind_color_index,
    bind_flip_x,
    bind_flip_y,
}
