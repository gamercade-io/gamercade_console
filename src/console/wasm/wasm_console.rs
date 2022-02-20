use std::sync::Arc;

use ggrs::GGRSRequest;
use parking_lot::Mutex;
use wasmer::{
    Exports, Extern, Function, ImportObject, Instance, Module, Mutability, NativeFunc, Store,
};

use super::network::{SaveStateDefinition, WasmConsoleState};
use crate::{
    api::{GraphicsApiBinding, InputApiBinding},
    console::{GraphicsContext, InputContext},
    core::{PlayerInputEntry, Rom},
    Console,
};

pub struct WasmConsole {
    rom: Arc<Rom>,
    functions: Functions,
    instance: Instance,
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) input_context: InputContext,
    state_definition: SaveStateDefinition,
}

#[derive(Clone)]
pub(crate) struct Functions {
    init_fn: NativeFunc,
    update_fn: NativeFunc,
    draw_fn: NativeFunc,
}

impl Functions {
    pub(crate) fn find_functions(instance: &Instance) -> Self {
        let init_fn = instance
            .exports
            .get_function("init")
            .unwrap()
            .native::<(), ()>()
            .unwrap();

        let update_fn = instance
            .exports
            .get_function("update")
            .unwrap()
            .native::<(), ()>()
            .unwrap();

        let draw_fn = instance
            .exports
            .get_function("draw")
            .unwrap()
            .native::<(), ()>()
            .unwrap();

        Self {
            init_fn,
            update_fn,
            draw_fn,
        }
    }
}

pub(crate) struct WasmConsoleBuilder<'a> {
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) input_context: InputContext,
    pub(crate) store: &'a Store,
    pub(crate) imports: Vec<(&'static str, Function)>,
}

impl WasmConsoleBuilder<'_> {
    fn build_import_object(mut self) -> ImportObject {
        self.bind_graphics_api();
        self.bind_input_api();

        let mut output = ImportObject::new();
        let mut namespace = Exports::new();

        self.imports.into_iter().for_each(|(name, function)| {
            namespace.insert(name, function);
        });

        output.register("env", namespace);

        output
    }
}

impl WasmConsole {
    pub fn new(
        rom: Arc<Rom>,
        input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
        code: &[u8],
        frame_buffer: Arc<Mutex<Box<[u8]>>>,
    ) -> Self {
        // Initialize the contexts
        let graphics_context = GraphicsContext {
            frame_buffer,
            rom: rom.clone(),
        };
        let input_context = InputContext { input_entries };
        let store = Store::default();
        let module = Module::new(&store, code).unwrap();

        let import_object = WasmConsoleBuilder {
            graphics_context: graphics_context.clone(),
            input_context: input_context.clone(),
            store: &store,
            imports: Vec::new(),
        }
        .build_import_object();

        let instance = Instance::new(&module, &import_object).unwrap();
        let functions = Functions::find_functions(&instance);

        let mut memories = Vec::new();
        let mut mutable_globals = Vec::new();

        instance
            .exports
            .iter()
            .for_each(|(name, export)| match export {
                Extern::Global(global) => {
                    if global.ty().mutability == Mutability::Var {
                        mutable_globals.push(name.clone())
                    }
                }
                Extern::Memory(_) => memories.push(name.clone()),
                Extern::Function(_) => (),
                Extern::Table(_) => (),
            });

        let state_definition = SaveStateDefinition {
            memories,
            mutable_globals,
        };

        Self {
            rom,
            graphics_context,
            input_context,
            functions,
            instance,
            state_definition,
        }
    }

    fn generate_save_state(&self) -> WasmConsoleState {
        let previous_buttons = self
            .input_context
            .input_entries
            .lock()
            .iter()
            .map(|input| input.previous)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let memories = self
            .state_definition
            .memories
            .iter()
            .map(|name| unsafe {
                self.instance
                    .exports
                    .get_memory(name)
                    .unwrap()
                    .data_unchecked()
                    .to_vec()
            })
            .collect();

        let mutable_globals = self
            .state_definition
            .mutable_globals
            .iter()
            .map(|name| self.instance.exports.get_global(name).unwrap().get())
            .collect();

        WasmConsoleState {
            previous_buttons,
            memories,
            mutable_globals,
        }
    }

    fn load_save_state(&mut self, state: WasmConsoleState) {
        let WasmConsoleState {
            previous_buttons,
            memories,
            mutable_globals,
        } = state;

        let mut lock = self.input_context.input_entries.lock();
        previous_buttons
            .iter()
            .enumerate()
            .for_each(|(index, prev)| {
                lock[index].previous = *prev;
            });
        drop(lock);

        self.state_definition
            .memories
            .iter()
            .enumerate()
            .for_each(|(index, name)| unsafe {
                let source = &memories[index];
                let destination = self.instance.exports.get_memory(name).unwrap();
                let destination = &mut destination.data_unchecked_mut()[..source.len()];
                destination.copy_from_slice(source)
            });

        self.state_definition
            .mutable_globals
            .iter()
            .enumerate()
            .for_each(|(index, name)| {
                let source = mutable_globals[index].clone();
                self.instance
                    .exports
                    .get_global(name)
                    .unwrap()
                    .set(source)
                    .unwrap()
            })
    }
}

impl Console for WasmConsole {
    fn call_init(&self) {
        self.functions.init_fn.call().unwrap();
    }

    fn call_update(&self) {
        self.functions.update_fn.call().unwrap();
    }

    fn call_draw(&self) {
        self.functions.draw_fn.call().unwrap();
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.graphics_context.frame_buffer.lock());
    }

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>) {
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => {
                    let state = self.generate_save_state();
                    cell.save(frame, Some(state), None);
                }
                GGRSRequest::LoadGameState { cell, .. } => {
                    let state = cell.load().expect("Failed to load game state");
                    self.load_save_state(state);
                }
                GGRSRequest::AdvanceFrame { inputs } => {
                    // Copy new inputs into the state
                    let mut lock = self.input_context.input_entries.lock();
                    lock.iter_mut()
                        .zip(inputs.iter())
                        .for_each(|(current, new)| {
                            current.current = new.0;
                        });
                    drop(lock);

                    // Call update
                    self.call_update();

                    // Advance the input data
                    let mut lock = self.input_context.input_entries.lock();
                    lock.iter_mut().for_each(|inputs| {
                        inputs.previous = inputs.current.buttons;
                    });
                }
            }
        }
    }
}
