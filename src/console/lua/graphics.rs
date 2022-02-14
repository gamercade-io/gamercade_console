use rlua::{Context, UserData};

use super::{LuaConsole, LUA_RENDER_CONTEXT};
use crate::{
    api::{GraphicsApi, GraphicsApiBinding},
    console::GraphicsContext,
};

impl GraphicsApiBinding for LuaConsole {
    fn bind_clear_screen(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "clear_screen",
                    ctx.create_function(|inner_ctx, args: (Option<usize>, Option<usize>)| {
                        get_graphics_context(&inner_ctx).clear_screen(args.0, args.1);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        });
    }

    fn bind_set_pixel(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "set_pixel",
                    ctx.create_function(
                        |inner_ctx, args: (u32, u32, Option<usize>, Option<usize>)| {
                            get_graphics_context(&inner_ctx)
                                .set_pixel(args.0, args.1, args.2, args.3);
                            Ok(())
                        },
                    )
                    .unwrap(),
                )
                .unwrap();
        })
    }

    fn bind_height(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "height",
                    ctx.create_function(|inner_ctx, ()| {
                        Ok(get_graphics_context(&inner_ctx).height())
                    })
                    .unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_width(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "width",
                    ctx.create_function(|inner_ctx, ()| {
                        Ok(get_graphics_context(&inner_ctx).width())
                    })
                    .unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_line(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "line",
                    ctx.create_function(
                        |inner_ctx, args: (u32, u32, u32, u32, Option<usize>, Option<usize>)| {
                            get_graphics_context(&inner_ctx)
                                .line(args.0, args.1, args.2, args.3, args.4, args.5);
                            Ok(())
                        },
                    )
                    .unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_rect(&mut self) {
        self.lua.context(|ctx| {
            ctx.globals()
                .set(
                    "rect",
                    ctx.create_function(
                        |inner_ctx, args: (u32, u32, u32, u32, Option<usize>, Option<usize>)| {
                            get_graphics_context(&inner_ctx)
                                .rect(args.0, args.1, args.2, args.3, args.4, args.5);
                            Ok(())
                        },
                    )
                    .unwrap(),
                )
                .unwrap()
        })
    }
}

fn get_graphics_context(context: &Context) -> GraphicsContext {
    context
        .named_registry_value::<_, GraphicsContext>(LUA_RENDER_CONTEXT)
        .unwrap()
}

impl UserData for GraphicsContext {}
