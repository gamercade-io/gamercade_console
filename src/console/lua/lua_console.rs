use std::sync::Arc;

use ggrs::GGRSRequest;
use parking_lot::Mutex;
use rlua::{Function, Lua, RegistryKey, Table, Number, Value};

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
    pub(crate) states: RegistryKey,
    max_rollback_frames: i32,
}

impl Console for LuaConsole {
    fn call_init(&self) {
        // Call the rom's init function
        self.lua.context(|ctx| {
            let init: Function = ctx.globals().get("init").unwrap();
            init.call::<_, ()>(()).unwrap();

            let env: Table = ctx.load("_ENV").eval().unwrap();
            for pair in env.pairs::<Value, Value>() {
                if let Ok((key, value)) = pair {
                    println!("{:#?}: {:#?}", key, value);
                }
            }
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
                    let frame = frame % self.max_rollback_frames;
                    self.lua.context(|ctx| {
                        let states_table: Table = ctx.registry_value(&self.states).unwrap();
                        states_table
                            .set(
                                frame,
                                ctx.load("__deepcopy__(_ENV)").eval::<Table>().unwrap(),
                            )
                            .unwrap();
                    })
                }
                GGRSRequest::LoadGameState { cell, frame } => {
                    //TODO: Actually load the game state for rollbacks
                    let frame = frame % self.max_rollback_frames;
                    let mut lock = self.input_entries.lock();
                    println!("load: {}", frame);
                    *lock = cell.load().unwrap();
                    self.lua.context(|ctx| {
                        let prev_x: Number = ctx.globals().get("X_POS").unwrap();
                        let prev_y: Number = ctx.globals().get("Y_POS").unwrap();
                        println!("before: {}, {}", prev_x, prev_y);

                        let states_table: Table = ctx.registry_value(&self.states).unwrap();
                        for pair in states_table.pairs::<Value, Value>() {
                            let (key, value) = pair.unwrap();
                            println!("{:?}: {:?}", key, value);
                        }

                        let states_table: Table = ctx.registry_value(&self.states).unwrap();
                        let deep_copy: Function = ctx.globals().get("__deepcopy__").unwrap();

                        let rollback: Table = states_table.get(frame).unwrap();
                        let copied = deep_copy.bind(rollback).unwrap().call::<(), Table>(()).unwrap();
                        ctx.load(SET_ENV).call::<Table, ()>(copied).unwrap();

                        let new_x: Number = ctx.globals().get("X_POS").unwrap();
                        let new_y: Number = ctx.globals().get("Y_POS").unwrap();
                        println!("after: {}, {}", new_x, new_y);

                    })
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
        max_rollback_frames: usize,
    ) -> Self {
        let lua = Lua::new();
        let max_rollback_frames = (max_rollback_frames + 1) as i32;

        let input_context = InputContext {
            input_entries: input_entries.clone(),
        };

        let (gfx, inp, states) = lua.context(|ctx| {
            // Load the user lua scripts
            ctx.load(code).exec().unwrap();
            ctx.load(DEEP_COPY).exec().unwrap();

            // Set the graphics context pointer
            let gfx = ctx
                .create_registry_value(GraphicsContext {
                    frame_buffer: frame_buffer.clone(),
                    rom: rom.clone(),
                })
                .unwrap();

            // Set the input context pointer
            let inp = ctx.create_registry_value(input_context).unwrap();

            // Set the "States" context pointer
            let states_table = ctx.create_table().unwrap();
            let states = ctx.create_registry_value(states_table).unwrap();

            (gfx, inp, states)
        });

        let mut output = Self {
            rom,
            lua,
            frame_buffer,
            input_entries,
            gfx,
            inp,
            states,
            max_rollback_frames,
        };

        output.bind_graphics_api();
        output.bind_input_api();
        output
    }
}

const DEEP_COPY: &str = r#"
function __deepcopy__(orig, copies)
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
                copy[__deepcopy__(orig_key, copies)] = __deepcopy__(orig_value, copies)
            end
            setmetatable(copy, __deepcopy__(getmetatable(orig), copies))
        end
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end"#;

const SET_ENV: &str = r#"
function __SET_ENV__(new)
    _ENV = new
end"#;
