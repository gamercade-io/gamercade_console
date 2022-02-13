use parking_lot::Mutex;
use std::sync::Arc;

use crate::{
    api::{GraphicsApi, GraphicsApiBinding},
    core::Rom,
};
use rlua::{Context, Function, Lua, UserData};

use super::{graphics_context::GraphicsContext, Console};

static LUA_RENDER_CONTEXT: &str = "__LUA_RENDER_CONTEXT__";

pub struct LuaConsole {
    rom: Arc<Rom>,
    lua: Lua,
    player_count: usize,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
}

impl Console for LuaConsole {
    fn call_input(&self, button_pressed: bool) {
        // Call the roms handle_input function for each player
        self.lua.context(|ctx| {
            let input: Function = ctx.globals().get("input").unwrap();
            (0..self.player_count).for_each(|player_id| {
                input
                    .call::<(usize, bool), ()>((player_id + 1, button_pressed))
                    .unwrap()
            });
        });
    }

    fn call_update(&self) {
        // Call the rom's update function
        self.lua.context(|ctx| {
            let update: Function = ctx.globals().get("update").unwrap();
            update.call::<_, ()>(()).unwrap();
        });
    }

    fn call_draw(&self) {
        // Call the rom's draw function
        self.lua.context(|ctx| {
            let draw: Function = ctx.globals().get("draw").unwrap();
            draw.call::<_, ()>(()).unwrap();
        });
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.frame_buffer.lock());
    }
}

impl LuaConsole {
    pub fn new(rom: Arc<Rom>, player_count: usize, code: &str) -> Self {
        let frame_buffer = (0..rom.resolution.total_pixels() * 4)
            .map(|_| 0)
            .collect::<Vec<u8>>()
            .into_boxed_slice();
        let frame_buffer = Arc::new(Mutex::new(frame_buffer));

        let lua = Lua::new();

        lua.context(|ctx| {
            // Load the user lua scripts
            ctx.load(code).exec().unwrap();
            ctx.set_named_registry_value(
                LUA_RENDER_CONTEXT,
                GraphicsContext {
                    frame_buffer: frame_buffer.clone(),
                    rom: rom.clone(),
                },
            )
            .unwrap();
        });

        let mut output = Self {
            rom,
            lua,
            player_count,
            frame_buffer,
        };

        output.bind_graphics_api();
        output
    }
}

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
                            Ok(get_graphics_context(&inner_ctx)
                                .line(args.0, args.1, args.2, args.3, args.4, args.5))
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
                            Ok(get_graphics_context(&inner_ctx)
                                .rect(args.0, args.1, args.2, args.3, args.4, args.5))
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
