macro_rules! derive_bind_graphics_api {
    ($($name:ident,)*) => {
        pub trait GraphicsApiBinding {
            $(fn $name(&mut self);)*

            fn bind_graphics_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

pub trait GraphicsApi {
    fn clear_screen(&self, color_index: Option<usize>, palette_index: Option<usize>);
    fn set_pixel(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>);
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

derive_bind_graphics_api! {
    bind_clear_screen,
    bind_set_pixel,
    bind_height,
    bind_width,
}
