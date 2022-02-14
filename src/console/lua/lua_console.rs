use std::sync::Arc;

use parking_lot::Mutex;
use rlua::{Function, Lua, Table};

use super::{Console, ToLuaTable, LUA_RENDER_CONTEXT};
use crate::{
    api::GraphicsApiBinding,
    console::GraphicsContext,
    core::{InputState, Rom},
};

pub struct LuaConsole {
    rom: Arc<Rom>,
    pub(crate) lua: Lua,
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
