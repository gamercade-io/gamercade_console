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

    //TODO
    // fn circle(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>);
    // fn circle_filled(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>);

    // fn rect(&self, x: u32, y: u32, width: u32, height: u32, color_index: Option<usize>, palette_index: Option<usize>)
    // fn rect_filled(&self, x: u32, y: u32, width: u32, height: u32, color_index: Option<usize>, palette_index: Option<usize>)
    // fn line(&self, x1: u32, y1: u32, x2: u32, y2: u32, color_index: Option<usize>, palette_index: Option<usize>)

    // fn sprite(&self, sprite_index: usize, x: u32, y: u32, palette_index: Option<usize>)
}

derive_bind_graphics_api! {
    bind_clear_screen,
    bind_set_pixel,
    bind_height,
    bind_width,
    //bind_circle
    //bind_circle_filled
    //bind_rect
    //bind_rect_filled
    //bind_line
    //bind_sprite
}
