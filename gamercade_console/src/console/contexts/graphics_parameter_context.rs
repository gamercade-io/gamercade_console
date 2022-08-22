use gamercade_core::GraphicsParameters;

#[derive(Default)]
pub struct GraphicsParameterContext;

macro_rules! impl_graphics_parameter_context {
    ($($name:ident $ty:ty,)*)=> {
        impl GraphicsParameterContext {
            $(
                pub fn $name(&self, value: i32) -> i32 {
                    if let Ok(valid) = value.try_into() {
                        i32::from(GraphicsParameters::default().$name(valid))
                    } else {
                        0
                    }
                }
            )*
        }
    };
}

impl_graphics_parameter_context! {
    palette_index u8,
    sprite_sheet_index u8,
    sprite_index u8,
    color_index u8,
}

impl GraphicsParameterContext {
    pub fn flip_x(&self, flip: i32) -> i32 {
        i32::from(GraphicsParameters::default().flip_x(flip != 0))
    }

    pub fn flip_y(&self, flip: i32) -> i32 {
        i32::from(GraphicsParameters::default().flip_y(flip != 0))
    }

    pub fn graphics_parameters(
        &self,
        palette_index: i32,
        sprite_sheet_index: i32,
        sprite_index: i32,
        color_index: i32,
        flip_x: i32,
        flip_y: i32,
    ) -> i32 {
        GraphicsParameters::default()
            .palette_index(palette_index.try_into().unwrap_or_default())
            .sprite_sheet_index(sprite_sheet_index.try_into().unwrap_or_default())
            .sprite_index(sprite_index.try_into().unwrap_or_default())
            .color_index(color_index.try_into().unwrap_or_default())
            .flip_x(flip_x != 0)
            .flip_y(flip_y != 0)
            .into()
    }
}
