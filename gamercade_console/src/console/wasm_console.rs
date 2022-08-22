use std::sync::Arc;

use gamercade_audio::{SoundEngine, SoundRomInstance};
use ggrs::GGRSRequest;
use wasmtime::{Engine, ExternType, Instance, Linker, Module, Mutability, Store, TypedFunc};

type GameFunc = TypedFunc<(), ()>;

use super::{
    bindings,
    network::{SaveStateDefinition, WasmConsoleState},
    Contexts, SessionDescriptor,
};
use crate::Console;
use gamercade_core::Rom;

pub struct WasmConsole {
    pub(crate) rom: Arc<Rom>,
    pub(crate) store: Store<Contexts>,
    pub(crate) functions: Functions,
    pub(crate) instance: Instance,
    pub(crate) state_definition: SaveStateDefinition,
}

#[derive(Clone)]
pub(crate) struct Functions {
    init_fn: Option<GameFunc>,
    update_fn: Option<GameFunc>,
    draw_fn: Option<GameFunc>,
}

impl Functions {
    pub(crate) fn find_functions<T>(store: &mut Store<T>, instance: &Instance) -> Self {
        let init_fn = match instance.get_typed_func(&mut *store, "init") {
            Ok(init_fn) => Some(init_fn),
            Err(e) => {
                println!("init function not found: {}", e);
                None
            }
        };
        let update_fn = match instance.get_typed_func(&mut *store, "update") {
            Ok(update_fn) => Some(update_fn),
            Err(e) => {
                println!("update function not found: {}", e);
                None
            }
        };
        let draw_fn = match instance.get_typed_func(&mut *store, "draw") {
            Ok(draw_fn) => Some(draw_fn),
            Err(e) => {
                println!("draw function not found: {}", e);
                None
            }
        };

        if init_fn.is_some() || update_fn.is_some() || draw_fn.is_some() {
            Self {
                init_fn,
                update_fn,
                draw_fn,
            }
        } else {
            panic!("Loaded rom doesn't contain any valid functions.")
        }
    }
}

impl WasmConsole {
    pub fn new(rom: Rom, seed: u64, session: SessionDescriptor) -> Self {
        // Initialize sound output
        // TODO: Update this with the latest changes
        //let sound_engine = SoundEngine::new(&rom.sounds);

        let rom = Arc::new(rom);
        let sound_rom = Arc::new(SoundRomInstance::new(&rom.sounds));

        // Initialize the contexts
        let contexts = Contexts::new(&rom, seed, session, &sound_rom);
        let engine = Engine::default();
        let module = Module::new(&engine, &rom.code).unwrap();
        let mut linker = Linker::new(&engine);

        // TODO: Make this static? Is there a way we can not have to call this
        // each time?
        bindings::bind_all_apis(&mut linker);

        let mut store = Store::new(&engine, contexts);
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let functions = Functions::find_functions(&mut store, &instance);

        let mut memories = Vec::new();
        let mut mutable_globals = Vec::new();

        module.exports().for_each(|export| {
            let name = export.name();
            match export.ty() {
                ExternType::Global(global) => {
                    if global.mutability() == Mutability::Var {
                        mutable_globals.push(name.to_string())
                    }
                }
                ExternType::Memory(_) => memories.push(name.to_string()),
                ExternType::Func(_) => (),
                ExternType::Table(_) => (),
            }
        });

        let state_definition = SaveStateDefinition {
            memories,
            mutable_globals,
        };

        let mut out = Self {
            rom,
            functions,
            instance,
            state_definition,
            store,
        };

        out.call_init();

        out
    }

    fn generate_save_state(&mut self) -> WasmConsoleState {
        let previous_buttons = self
            .store
            .data()
            .input_context
            .input_entries
            .iter()
            .map(|input| input.previous)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let memories = self
            .state_definition
            .memories
            .iter()
            .map(|name| {
                self.instance
                    .get_memory(&mut self.store, name)
                    .unwrap()
                    .data(&self.store)
                    .to_vec()
            })
            .collect();

        let mutable_globals = self
            .state_definition
            .mutable_globals
            .iter()
            .map(|name| self.instance.get_global(&mut self.store, name).unwrap())
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

        previous_buttons
            .iter()
            .enumerate()
            .for_each(|(index, prev)| {
                self.store.data_mut().input_context.input_entries[index].previous = *prev;
            });

        self.state_definition
            .memories
            .iter()
            .enumerate()
            .for_each(|(index, name)| {
                let source = &memories[index];
                let destination = self.instance.get_memory(&mut self.store, name).unwrap();
                let destination = &mut destination.data_mut(&mut self.store)[..source.len()];
                destination.copy_from_slice(source)
            });

        self.state_definition
            .mutable_globals
            .iter()
            .enumerate()
            .for_each(|(index, name)| {
                let source = mutable_globals[index];
                let val = source.get(&mut self.store);
                self.instance
                    .get_global(&mut self.store, name)
                    .unwrap()
                    .set(&mut self.store, val)
                    .unwrap()
            })
    }
}

fn call<T>(func: &Option<GameFunc>, store: &mut Store<T>) {
    if let Some(func) = func {
        func.call(store, ()).unwrap()
    }
}

impl Console for WasmConsole {
    fn call_init(&mut self) {
        call(&self.functions.init_fn, &mut self.store);
    }

    fn call_update(&mut self) {
        call(&self.functions.update_fn, &mut self.store);
    }

    fn call_draw(&mut self) {
        call(&self.functions.draw_fn, &mut self.store);
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.store.data().draw_context.frame_buffer.pixel_buffer);
    }

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>) {
        // TODO: Figure out something to handle audio needing to
        // roll back/forward
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => {
                    // TODO: Fire off request to sound thread
                    let state = self.generate_save_state();
                    // TODO: Collect results from sound thread
                    cell.save(frame, Some(state), None);
                }
                GGRSRequest::LoadGameState { cell, .. } => {
                    let state = cell.load().expect("Failed to load game state");
                    // TODO: Fire off sync to sound thread
                    self.load_save_state(state);
                }
                GGRSRequest::AdvanceFrame { inputs } => {
                    // Copy new inputs into the state
                    self.store
                        .data_mut()
                        .input_context
                        .input_entries
                        .iter_mut()
                        .zip(inputs.iter())
                        .for_each(|(current, new)| {
                            current.current = new.0;
                        });

                    // Call update
                    self.call_update();

                    // Advance the input data
                    self.store
                        .data_mut()
                        .input_context
                        .input_entries
                        .iter_mut()
                        .for_each(|inputs| {
                            inputs.previous = inputs.current.buttons;
                        });
                }
            }
        }
    }
}
