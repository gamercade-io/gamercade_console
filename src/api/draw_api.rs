macro_rules! derive_bind_draw_api {
    ($($name:ident,)*) => {
        pub trait DrawApiBinding {
            $(fn $name(&mut self);)*

            fn bind_draw_api(&mut self) {
                $(self.$name();)*
            }
        }
    };
}

pub trait DrawApi {
    fn clear_screen(&mut self, color_index: i32, palette_index: i32);
    fn set_pixel(&mut self, x: i32, y: i32, color_index: i32, palette_index: i32);
    fn height(&self) -> i32;
    fn width(&self) -> i32;

    //TODO
    fn circle(&mut self, x: i32, y: i32, radius: i32, color_index: i32, palette_index: i32);
    // fn circle_filled(&self, x: i32, y: i32, color_index: i32, palette_index: i32);

    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    );

    fn rect_filled(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    );
    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color_index: i32, palette_index: i32);

    // fn sprite(&self, sprite_index: i32, x: i32, y: i32, palette_index: i32)
}

derive_bind_draw_api! {
    bind_clear_screen,
    bind_set_pixel,
    bind_height,
    bind_width,
    bind_circle,
    //bind_circle_filled,
    bind_rect,
    bind_rect_filled,
    bind_line,
    //bind_sprite,
}
