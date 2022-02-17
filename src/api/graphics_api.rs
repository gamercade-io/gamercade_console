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
    fn clear_screen(&self, color_index: i32, palette_index: i32);
    fn set_pixel(&self, x: i32, y: i32, color_index: i32, palette_index: i32);
    fn height(&self) -> i32;
    fn width(&self) -> i32;

    //TODO
    // fn circle(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>);
    // fn circle_filled(&self, x: u32, y: u32, color_index: Option<usize>, palette_index: Option<usize>);

    fn rect(&self, x: i32, y: i32, width: i32, height: i32, color_index: i32, palette_index: i32);

    // fn rect_filled(&self, x: u32, y: u32, width: u32, height: u32, color_index: Option<usize>, palette_index: Option<usize>)
    fn line(&self, x0: i32, y0: i32, x1: i32, y1: i32, color_index: i32, palette_index: i32);

    // fn sprite(&self, sprite_index: usize, x: u32, y: u32, palette_index: Option<usize>)
}

derive_bind_graphics_api! {
    bind_clear_screen,
    bind_set_pixel,
    bind_height,
    bind_width,
    //bind_circle,
    //bind_circle_filled,
    bind_rect,
    //bind_rect_filled,
    bind_line,
    //bind_sprite,
}
