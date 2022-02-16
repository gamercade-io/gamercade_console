use std::sync::Arc;

use ggrs::GGRSRequest;
use parking_lot::Mutex;
use rlua::{Function, Lua, RegistryKey, Table};

use super::Console;
use crate::{
    api::{GraphicsApiBinding, InputApiBinding},
    console::{GraphicsContext, InputContext},
    core::{PlayerInputEntry, Rom},
    GGRSConfig,
};

pub struct LuaConsole {
    rom: Arc<Rom>,
    pub(crate) lua: Lua,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
    input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
    pub(crate) gfx: RegistryKey,
    pub(crate) inp: RegistryKey,
}

impl Console for LuaConsole {
    fn call_init(&self) {
        // Call the rom's init function
        self.lua.context(|ctx| {
            let init: Function = ctx.globals().get("init").unwrap();
            init.call::<_, ()>(()).unwrap();

            // ctx.load(DEEP_COPY).exec().unwrap();
            // println!("loaded deepcopy");

            // let snapshot_table = ctx.create_registry_value(ctx.create_table().unwrap()).unwrap()

            // let new_env: Table = ctx.load(r#"
            // __SNAPS__ = {}
            // new_env = deepcopy(_ENV)
            // _ENV = new_env
            // "#).eval().unwrap();
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

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<GGRSConfig>>) {
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

        let (gfx, inp) = lua.context(|ctx| {
            // Load the user lua scripts
            ctx.load(code).exec().unwrap();

            // Set the graphics context pointer
            let gfx = ctx
                .create_registry_value(GraphicsContext {
                    frame_buffer: frame_buffer.clone(),
                    rom: rom.clone(),
                })
                .unwrap();

            // Set the input context pointer
            let inp = ctx.create_registry_value(input_context).unwrap();

            (gfx, inp)
        });

        let mut output = Self {
            rom,
            lua,
            frame_buffer,
            input_entries,
            gfx,
            inp,
        };

        output.bind_graphics_api();
        output.bind_input_api();
        output
    }
}

const DEEP_COPY: &str = r#"
function deepcopy(orig, copies)
    copies = copies or {}
    local orig_type = type(orig)
    local copy
    if orig_type == 'table' then
        if copies[orig] then
            copy = copies[orig]
        else
            copy = {}
            copies[orig] = copy
            for orig_key, orig_value in next, orig, nil do
                copy[deepcopy(orig_key, copies)] = deepcopy(orig_value, copies)
            end
            setmetatable(copy, deepcopy(getmetatable(orig), copies))
        end
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end"#;
