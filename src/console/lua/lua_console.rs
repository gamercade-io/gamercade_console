use std::sync::Arc;

use ggrs::GGRSRequest;
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
    input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
}

impl Console for LuaConsole {
    fn call_init(&self) {
        // Call the rom's init function
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

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<crate::GGRSConfig>>) {
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => {
                    //TODO: Actually save the game state for rollbacks
                    cell.save(frame, Some(self.input_entries.lock().clone()), None);
                }
                GGRSRequest::LoadGameState { cell, frame: _ } => {
                    //TODO: Actually load the game state for rollbacks
                    let mut lock = self.input_entries.lock();
                    *lock = cell.load().unwrap();
                }
                GGRSRequest::AdvanceFrame { inputs } => {
                    let mut lock = self.input_entries.lock();

                    for (index, (next_state, _status)) in inputs.iter().enumerate() {
                        lock[index].push_input_state(*next_state);
                    }
                    drop(lock);

                    self.call_update()
                }
            }
        }
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

        let input_context = InputContext {
            input_entries: input_entries.clone(),
        };

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

            ctx.set_named_registry_value(LUA_INPUT_CONTEXT, input_context)
                .unwrap();
        });

        let mut output = Self {
            rom,
            lua,
            frame_buffer,
            input_entries,
        };

        output.bind_graphics_api();
        output.bind_input_api();
        output
    }
}
