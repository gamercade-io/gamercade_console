use std::sync::Arc;

use gamercade_sound_engine::{SoundEngine, SoundEngineData, SoundRomInstance};
use ggrs::GGRSRequest;
use wasmtime::{Engine, ExternType, Instance, Linker, Module, Mutability, Store, TypedFunc};
use winit::{dpi::PhysicalPosition, window::Window};

type GameFunc = TypedFunc<(), ()>;

use super::{
    bindings,
    network::{SaveStateDefinition, WasmConsoleState},
    Contexts, SessionDescriptor,
};
use crate::Console;
use gamercade_fs::Rom;

pub struct WasmConsole {
    pub(crate) rom: Arc<Rom>,
    pub(crate) store: Store<Contexts>,
    pub(crate) functions: Functions,
    pub(crate) instance: Instance,
    pub(crate) state_definition: SaveStateDefinition,
    pub(crate) sound_engine: SoundEngine,
    pub(crate) audio_out: SoundEngineData,
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
                println!("init function not found: {e}");
                None
            }
        };
        let update_fn = match instance.get_typed_func(&mut *store, "update") {
            Ok(update_fn) => Some(update_fn),
            Err(e) => {
                println!("update function not found: {e}");
                None
            }
        };
        let draw_fn = match instance.get_typed_func(&mut *store, "draw") {
            Ok(draw_fn) => Some(draw_fn),
            Err(e) => {
                println!("draw function not found: {e}");
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
    pub fn new(
        rom: Rom,
        seed: u64,
        session: SessionDescriptor,
        max_prediction: usize,
    ) -> (Self, WasmConsoleState) {
        // Initialize sound output

        let rom = Arc::new(rom);
        let sound_rom = Arc::new(SoundRomInstance::new(&rom.sounds));

        let sound_engine = SoundEngine::new(
            rom.frame_rate.frames_per_second(),
            &sound_rom,
            max_prediction,
        );
        let output_sample_rate = sound_engine.output_sample_rate();

        // Initialize the contexts
        let contexts = Contexts::new(&rom, seed, session, &sound_rom, output_sample_rate);
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

        let audio_out = store.data().audio_context.sound_engine_data.clone();

        let mut out = Self {
            rom,
            functions,
            instance,
            state_definition,
            store,
            sound_engine,
            audio_out,
        };

        out.call_init();

        out.sync_audio();

        let initial_state = out.generate_save_state();

        (out, initial_state)
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

        let sound_engine_data = self.store.data().audio_context.sound_engine_data.clone();

        WasmConsoleState {
            previous_buttons,
            memories,
            mutable_globals,
            sound_engine_data,
        }
    }

    pub fn load_save_state(&mut self, state: WasmConsoleState) {
        let WasmConsoleState {
            previous_buttons,
            memories,
            mutable_globals,
            sound_engine_data,
        } = state;

        let audio_context = &mut self.store.data_mut().audio_context;
        audio_context.sound_engine_data = sound_engine_data;
        audio_context.changed = true;

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
            });
    }

    pub(crate) fn sync_audio(&mut self) {
        if self.store.data_mut().audio_context.changed {
            self.sound_engine.sync_audio_thread(&self.audio_out);
            self.store.data_mut().audio_context.changed = false;
        }
    }

    pub(crate) fn sync_mouse(&mut self, window: &Window) {
        match self.store.data().input_context.mouse_locked {
            true => {
                let position = window.inner_size();
                window
                    .set_cursor_position(PhysicalPosition::new(
                        position.width / 2,
                        position.height / 2,
                    ))
                    .unwrap();
                window.set_cursor_grab(true).unwrap();
                window.set_cursor_visible(false);
            }
            false => {
                window.set_cursor_grab(false).unwrap();
                window.set_cursor_visible(true);
            }
        }
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
                    self.store
                        .data_mut()
                        .input_context
                        .input_entries
                        .iter_mut()
                        .zip(inputs.iter())
                        .for_each(|(current, new)| {
                            current.current = new.0.input_state;
                            current.current_mouse = new.0.mouse_state;
                        });

                    // Call update
                    self.call_update();

                    // Store the "output audio" for when we need to render later
                    self.audio_out = self.store.data().audio_context.sound_engine_data.clone();

                    // Advance the audio data locally
                    self.sound_engine
                        .fast_forward(&mut self.store.data_mut().audio_context.sound_engine_data);

                    // Advance the input data
                    self.store
                        .data_mut()
                        .input_context
                        .input_entries
                        .iter_mut()
                        .for_each(|inputs| {
                            inputs.previous = inputs.current.buttons;
                            inputs.previous_mouse = inputs.current_mouse;
                        });
                }
            }
        }
    }
}
