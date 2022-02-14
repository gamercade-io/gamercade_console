use parking_lot::Mutex;
use std::sync::Arc;
use strum::IntoEnumIterator;

use crate::{
    api::{GraphicsApi, GraphicsApiBinding},
    core::{ButtonCode, Buttons, InputState, Rom},
};
use rlua::{Context, Function, Lua, Table, UserData};

use super::{graphics_context::GraphicsContext, Console};

static LUA_RENDER_CONTEXT: &str = "__LUA_RENDER_CONTEXT__";

pub struct LuaConsole {
    rom: Arc<Rom>,
    lua: Lua,
    player_count: usize,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
}

impl Console for LuaConsole {
    fn call_init(&self) {
        self.lua.context(|ctx| {
            let init: Function = ctx.globals().get("init").unwrap();
            init.call::<_, ()>(()).unwrap();
        });
    }
    fn call_update(&self, input_states: &[InputState]) {
        // Call the rom's update function
        self.lua.context(|ctx| {
            let update: Function = ctx.globals().get("update").unwrap();

            let input_array = ctx.create_table().unwrap();

            (0..self.player_count).for_each(|player_id| {
                input_array
                    .set(player_id + 1, input_states[player_id].to_lua_table(&ctx))
                    .unwrap();
            });

            update.call::<Table, ()>(input_array).unwrap();
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

trait ToLuaTable {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua>;
}

impl ToLuaTable for InputState {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua> {
        let output = ctx.create_table().unwrap();

        //TODO: Add stuff like .analogs, .triggers etc
        output
            .set("buttons", self.buttons.to_lua_table(ctx))
            .unwrap();

        output
    }
}

impl ToLuaTable for Buttons {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua> {
        let output = ctx.create_table().unwrap();

        ButtonCode::iter().for_each(|button| {
            output
                .set(button.to_lua_code(), self.get_button_state(button))
                .unwrap();
        });

        output
    }
}

impl ButtonCode {
    const S_UP: &'static str = "up";
    const S_DOWN: &'static str = "down";
    const S_LEFT: &'static str = "left";
    const S_RIGHT: &'static str = "right";
    const S_A: &'static str = "a";
    const S_B: &'static str = "b";
    const S_C: &'static str = "c";
    const S_D: &'static str = "d";
    const S_START: &'static str = "start";
    const S_SELECT: &'static str = "select";
    const S_LEFT_SHOULDER: &'static str = "lshoulder";
    const S_RIGHT_SHOULDER: &'static str = "rshoulder";
    const S_LEFT_STICK: &'static str = "rstick";
    const S_RIGHT_STICK: &'static str = "lstick";
    const S_LEFT_TRIGGER: &'static str = "ltrigger";
    const S_RIGHT_TRIGGER: &'static str = "rtrigger";
}

impl LuaCode for ButtonCode {
    fn to_lua_code(&self) -> &str {
        match self {
            Self::Up => Self::S_UP,
            Self::Down => Self::S_DOWN,
            Self::Left => Self::S_LEFT,
            Self::Right => Self::S_RIGHT,
            Self::A => Self::S_A,
            Self::B => Self::S_B,
            Self::C => Self::S_C,
            Self::D => Self::S_D,
            Self::Start => Self::S_START,
            Self::Select => Self::S_SELECT,
            Self::LeftShoulder => Self::S_LEFT_SHOULDER,
            Self::RightShoulder => Self::S_RIGHT_SHOULDER,
            Self::LeftStick => Self::S_LEFT_STICK,
            Self::RightStick => Self::S_RIGHT_STICK,
            Self::LeftTrigger => Self::S_LEFT_TRIGGER,
            Self::RightTrigger => Self::S_RIGHT_TRIGGER,
        }
    }
}

pub trait LuaCode: Sized {
    fn to_lua_code(&self) -> &str;
}
