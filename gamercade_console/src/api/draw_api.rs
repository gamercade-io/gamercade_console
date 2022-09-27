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
    fn clear_screen(&mut self, graphics_parameters: i32);
    fn set_pixel(&mut self, graphics_parameters: i32, x: i32, y: i32);

    fn circle(&mut self, graphics_parameters: i32, x: i32, y: i32, radius: i32);

    fn circle_filled(&mut self, graphics_parameters: i32, x: i32, y: i32, radius: i32);

    fn rect(&mut self, graphics_parameters: i32, x: i32, y: i32, width: i32, height: i32);

    fn rect_filled(&mut self, graphics_parameters: i32, x: i32, y: i32, width: i32, height: i32);

    fn line(&mut self, graphics_parameters: i32, x0: i32, y0: i32, x1: i32, y1: i32);

    fn sprite(&mut self, graphics_parameters: i32, transparency_mask: i64, x: i32, y: i32);

    fn write_pixel_buffer(&mut self, start_index: usize, data: &[u32]);
}

derive_bind_draw_api! {
    bind_clear_screen,
    bind_set_pixel,
    bind_circle,
    bind_circle_filled,
    bind_rect,
    bind_rect_filled,
    bind_line,
    bind_sprite,
    bind_write_pixel_buffer,
}
