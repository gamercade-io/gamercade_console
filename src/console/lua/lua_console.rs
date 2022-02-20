use std::sync::Arc;

use ggrs::GGRSRequest;
use parking_lot::Mutex;
use rlua::{Context, Function, Lua, RegistryKey, Table};

use super::Console;
use crate::{
    api::{GraphicsApiBinding, InputApiBinding},
    console::{GraphicsContext, InputContext},
    core::{PlayerInputEntry, Rom},
};

pub struct Registers {
    graphics: RegistryKey,
    input: RegistryKey,
    states: RegistryKey,
    update_fn: RegistryKey,
    update_blob: RegistryKey,
    draw_fn: RegistryKey,
    draw_blob: RegistryKey,
    //deep_copy_fn: RegistryKey,
}

impl Registers {
    pub(crate) fn get_graphics_context(&self, ctx: &Context) -> GraphicsContext {
        ctx.registry_value(&self.graphics).unwrap()
    }

    pub(crate) fn get_input_context(&self, ctx: &Context) -> InputContext {
        ctx.registry_value(&self.input).unwrap()
    }

    pub(crate) fn get_states<'a>(&self, ctx: &Context<'a>) -> Table<'a> {
        ctx.registry_value(&self.states).unwrap()
    }

    //pub(crate) fn get_deep_copy_fn<'a>(&self, ctx: &Context<'a>) -> Function<'a> {
    //    ctx.registry_value(&self.deep_copy_fn).unwrap()
    //}

    pub(crate) fn refresh_update_fn<'a>(&mut self, ctx: &Context<'a>, update_fn: Function<'a>) {
        let new_update_fn = ctx.create_registry_value(update_fn).unwrap();
        let to_replace = std::mem::replace(&mut self.update_fn, new_update_fn);
        ctx.remove_registry_value(to_replace).unwrap();
    }

    pub(crate) fn get_update_blob<'a>(&self, ctx: &Context<'a>) -> Vec<u8> {
        ctx.registry_value(&self.update_blob).unwrap()
    }

    pub(crate) fn get_update_fn<'a>(&self, ctx: &Context<'a>) -> Function<'a> {
        ctx.registry_value(&self.update_fn).unwrap()
    }

    pub(crate) fn refresh_draw_fn<'a>(&mut self, ctx: &Context<'a>, draw_fn: Function<'a>) {
        let new_draw_fn = ctx.create_registry_value(draw_fn).unwrap();
        let to_replace = std::mem::replace(&mut self.draw_fn, new_draw_fn);
        ctx.remove_registry_value(to_replace).unwrap();
    }

    pub(crate) fn get_draw_blob<'a>(&self, ctx: &Context<'a>) -> Vec<u8> {
        ctx.registry_value(&self.draw_blob).unwrap()
    }

    pub(crate) fn get_draw_fn<'a>(&self, ctx: &Context<'a>) -> Function<'a> {
        ctx.registry_value(&self.draw_fn).unwrap()
    }
}

pub struct LuaConsole {
    rom: Arc<Rom>,
    pub(crate) lua: Lua,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
    input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
    pub(crate) registers: Registers,
    max_rollback_frames: i32,
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
            let count: f64 = ctx.globals().get("COUNT").unwrap();
            let x_pos: usize = ctx.globals().get("X_POS").unwrap();
            let y_pos: usize = ctx.globals().get("Y_POS").unwrap();
            println!(
                "UPDATE BEFORE: count: {}, x: {}, y: {}",
                count, x_pos, y_pos
            );

            let update = self.registers.get_update_fn(&ctx);
            //let update: Function = ctx.globals().get("update").unwrap();
            update.call::<(), ()>(()).unwrap();

            let count: f64 = ctx.globals().get("COUNT").unwrap();
            let x_pos: usize = ctx.globals().get("X_POS").unwrap();
            let y_pos: usize = ctx.globals().get("Y_POS").unwrap();
            println!("UPDATE AFTER: count: {}, x: {}, y: {}", count, x_pos, y_pos);
        });
    }

    fn call_draw(&self) {
        // Call the rom's draw function
        self.lua.context(|ctx| {
            let draw = self.registers.get_draw_fn(&ctx);

            let count: f64 = ctx.globals().get("COUNT").unwrap();
            let x_pos: usize = ctx.globals().get("X_POS").unwrap();
            let y_pos: usize = ctx.globals().get("Y_POS").unwrap();
            println!("DRAW BEFORE: count: {}, x: {}, y: {}", count, x_pos, y_pos);

            //let draw: Function = ctx.globals().get("draw").unwrap();
            draw.call::<_, ()>(()).unwrap();

            let count: f64 = ctx.globals().get("COUNT").unwrap();
            let x_pos: usize = ctx.globals().get("X_POS").unwrap();
            let y_pos: usize = ctx.globals().get("Y_POS").unwrap();
            println!("DRAW AFTER: count: {}, x: {}, y: {}", count, x_pos, y_pos);
        });
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.frame_buffer.lock());
    }

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>) {
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => {
                    // Save the current input state
                    cell.save(frame, Some(self.input_entries.lock().clone()), None);

                    // Save the lua state into the state register
                    let frame = frame % self.max_rollback_frames;
                    self.lua.context(|ctx| {
                        let states_table: Table = self.registers.get_states(&ctx);
                        let cloned_state: Table = ctx.load(CLONE_STATE).eval().unwrap();

                        println!("SAVE:");
                        let count: f64 = ctx.globals().get("COUNT").unwrap();
                        let x_pos: usize = ctx.globals().get("X_POS").unwrap();
                        let y_pos: usize = ctx.globals().get("Y_POS").unwrap();
                        println!("globals: count: {}, x: {}, y: {}", count, x_pos, y_pos);

                        let count: f64 = cloned_state.get("COUNT").unwrap();
                        let x_pos: usize = cloned_state.get("X_POS").unwrap();
                        let y_pos: usize = cloned_state.get("Y_POS").unwrap();
                        println!("cloned count: {}, x: {}, y: {}", count, x_pos, y_pos);

                        states_table.set(frame, cloned_state).unwrap();
                    })
                }
                GGRSRequest::LoadGameState { cell, frame } => {
                    let frame = frame % self.max_rollback_frames;

                    // Rollback the input states
                    let mut lock = self.input_entries.lock();
                    *lock = cell.load().unwrap();

                    // Roll back the lua state
                    self.lua.context(|ctx| {
                        // Get the rollback state
                        let states_table: Table = self.registers.get_states(&ctx);
                        let rollback: Table = states_table.get(frame).unwrap();

                        //let copy_fn: Function = ctx
                        // .load(DEEP_COPY)
                        // .set_environment(rollback.clone())
                        // .unwrap()
                        // .eval()
                        // .unwrap();
                        //let rollback: Table = copy_fn.call(rollback).unwrap();

                        // Get the blobs
                        let update_blob: Vec<u8> = self.registers.get_update_blob(&ctx);
                        let draw_blob: Vec<u8> = self.registers.get_draw_blob(&ctx);

                        // Recompile them for the correct environment
                        let (update_fn, draw_fn) = unsafe {
                            let update_fn = ctx
                                .load(&update_blob)
                                .set_environment(rollback.clone())
                                .unwrap()
                                .into_function_allow_binary()
                                .unwrap();

                            let draw_fn = ctx
                                .load(&draw_blob)
                                .set_environment(rollback)
                                .unwrap()
                                .into_function_allow_binary()
                                .unwrap();

                            (update_fn, draw_fn)
                        };

                        // Update the registers with the new function
                        self.registers.refresh_update_fn(&ctx, update_fn);
                        self.registers.refresh_draw_fn(&ctx, draw_fn);
                    })
                }
                GGRSRequest::AdvanceFrame { inputs } => {
                    // Copy new inputs into the state
                    let mut lock = self.input_entries.lock();
                    lock.iter_mut()
                        .zip(inputs.iter())
                        .for_each(|(current, new)| {
                            current.current = new.0;
                        });
                    drop(lock);

                    // Call update
                    self.call_update();

                    // Advance the input data
                    let mut lock = self.input_entries.lock();
                    lock.iter_mut().for_each(|inputs| {
                        inputs.previous = inputs.current.buttons;
                    });
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

        let registers = lua.context(|ctx| {
            // Set the graphics context pointer
            let graphics = ctx
                .create_registry_value(GraphicsContext {
                    frame_buffer: frame_buffer.clone(),
                    rom: rom.clone(),
                })
                .unwrap();

            // Set the input context pointer
            let input = ctx.create_registry_value(input_context).unwrap();

            // Set the "States" context pointer
            let states_table = ctx.create_table().unwrap();
            let states = ctx.create_registry_value(states_table).unwrap();

            // Load the user lua scripts
            ctx.load(code).exec().unwrap();

            let update_fn: Function = ctx.globals().get("update").unwrap();
            let update_blob = update_fn.dump().unwrap();
            let draw_fn: Function = ctx.globals().get("draw").unwrap();
            let draw_blob = draw_fn.dump().unwrap();

            let update_fn = ctx.create_registry_value(update_fn).unwrap();
            let update_blob = ctx.create_registry_value(update_blob).unwrap();
            let draw_fn = ctx.create_registry_value(draw_fn).unwrap();
            let draw_blob = ctx.create_registry_value(draw_blob).unwrap();
            //let deep_copy_fn = ctx.create_registry_value(deep_copy_fn).unwrap();

            Registers {
                graphics,
                input,
                states,
                update_fn,
                update_blob,
                draw_fn,
                draw_blob,
                //deep_copy_fn,
            }
        });

        let mut output = Self {
            rom,
            lua,
            frame_buffer,
            input_entries,
            registers,
            max_rollback_frames,
        };

        output.bind_graphics_api();
        output.bind_input_api();
        output
    }
}

// const GRAPH_COPY: &str = r#"
// local function GraphCopy(x, seen)
//     seen = seen or {}

//     if seen[x] then
//         return seen[x]
//     end

//     if type(x) == 'table' then
//         local copy = {}
//         seen[x] = copy -- mark as seen BEFORE recursing deeper
//         for k, v in pairs(x) do
//             copy[GraphCopy(k, seen)] = GraphCopy(v, seen)
//         end
//         setmetatable(copy, GraphCopy(getmetatable(x), seen))
//         return copy
//     end

//     if type(x) == 'function' then
//         -- clone the function and GraphCopy each of its upvalues
//     end

//     return x
// end
// "#;

const CLONE_STATE: &str = r#"
function __DEEP_COPY__(orig, copies)
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
                copy[__DEEP_COPY__(orig_key, copies)] = __DEEP_COPY__(orig_value, copies)
            end
            setmetatable(copy, __DEEP_COPY__(getmetatable(orig), copies))
        end
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end
return __DEEP_COPY__(_ENV)"#;

// const DEEP_COPY: &str = r#"
// print('constructing __DEEP_COPY__')
// function __DEEP_COPY__(orig, copies)
//     copies = copies or {}
//     local orig_type = type(orig)
//     local copy
//     if orig_type == 'table' then
//         if copies[orig] then
//             copy = copies[orig]
//         else
//             copy = {}
//             copies[orig] = copy
//             for orig_key, orig_value in next, orig, nil do
//                 copy[__DEEP_COPY__(orig_key, copies)] = __DEEP_COPY__(orig_value, copies)
//             end
//             setmetatable(copy, __DEEP_COPY__(getmetatable(orig), copies))
//         end
//     else -- number, string, boolean, etc
//         copy = orig
//     end
//     return copy
// end
// return __DEEP_COPY__"#;
