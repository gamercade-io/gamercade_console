use std::sync::Arc;

use parking_lot::Mutex;
use rlua::{Function, Lua};

use super::{Console, LUA_INPUT_CONTEXT, LUA_RENDER_CONTEXT};
use crate::{
    api::{GraphicsApiBinding, InputApiBinding},
    console::{GraphicsContext, InputContext},
    core::{PlayerInputEntry, Rom},
};

pub struct LuaConsole {
    rom: Arc<Rom>,
    pub(crate) lua: Lua,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
}

impl Console for LuaConsole {
    fn call_init(&self) {
        self.lua.context(|ctx| {
            let init: Function = ctx.globals().get("init").unwrap();
            init.call::<_, ()>(()).unwrap();
        });
    }

    fn call_update(&self) {
        // Call the rom's update function
        self.lua.context(|ctx| {
            let update: Function = ctx.globals().get("update").unwrap();
            update.call::<(), ()>(()).unwrap();
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
    pub fn new(
        rom: Arc<Rom>,
        code: &str,
        frame_buffer: Arc<Mutex<Box<[u8]>>>,
        input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
    ) -> Self {
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

            ctx.set_named_registry_value(LUA_INPUT_CONTEXT, InputContext { input_entries })
                .unwrap();
        });

        let mut output = Self {
            rom,
            lua,
            frame_buffer,
        };

        output.bind_graphics_api();
        output.bind_input_api();
        output
    }
}
