use std::sync::Arc;

use gamercade_sound_engine::{SoundEngine, SoundEngineData, SoundRomInstance};
use ggrs::GgrsRequest;
use wasmtime::{Config, Engine, Instance, Linker, Module, Store, TypedFunc};
use winit::{
    dpi::PhysicalPosition,
    window::{CursorGrabMode, Window},
};

type GameFunc = TypedFunc<(), ()>;

pub const WASM_MEMORY: &str = "memory";

use super::{bindings, network::WasmConsoleState, Contexts, SessionDescriptor};
use crate::Console;
use gamercade_fs::Rom;

pub struct WasmConsole {
    pub(crate) rom: Arc<Rom>,
    pub(crate) store: Store<Contexts>,
    pub(crate) functions: Functions,
    pub(crate) instance: Instance,
    pub(crate) sound_engine: SoundEngine,
    pub(crate) audio_out: SoundEngineData,
    memory_values: MemoryValues,
}

#[derive(Default)]
struct MemoryValues {
    datapack_start: usize,
    datapack_end: usize,
}

#[derive(Clone)]
pub(crate) struct Functions {
    init_fn: Option<GameFunc>,
    datapack_fn: Option<TypedFunc<i32, i32>>,
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

        let datapack_fn = match instance.get_typed_func(&mut *store, "datapack") {
            Ok(datapack_fn) => Some(datapack_fn),
            Err(e) => {
                println!("datapack function not found: {e}");
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

        if init_fn.is_some() || datapack_fn.is_some() || update_fn.is_some() || draw_fn.is_some() {
            Self {
                init_fn,
                datapack_fn,
                update_fn,
                draw_fn,
            }
        } else {
            panic!("Loaded rom doesn't contain any valid functions.")
        }
    }
}

// TODO: Consider if "Host Memory" could
// improve the state management of the VM
fn wasmtime_config() -> Config {
    use wasmtime::*;
    let mut config = Config::new();

    config.async_support(false);
    config.async_stack_size(0);
    config.wasm_threads(false);
    config.wasm_tail_call(true);
    config.strategy(Strategy::Cranelift);
    config.cranelift_opt_level(OptLevel::Speed);
    config.static_memory_forced(true);

    config
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
        let engine = Engine::new(&wasmtime_config()).unwrap();
        let module = Module::new(&engine, &rom.code).unwrap();

        let mut linker = Linker::new(&engine);

        // TODO: Make this static? Is there a way we can not have to call this
        // each time?
        bindings::bind_all_apis(&mut linker);

        let mut store = Store::new(&engine, contexts);

        let instance = linker.instantiate(&mut store, &module).unwrap();

        let functions = Functions::find_functions(&mut store, &instance);

        let audio_out = store.data().audio_context.sound_engine_data.clone();

        let memory_values = MemoryValues::default();

        let mut out = Self {
            rom: rom.clone(),
            functions,
            instance,
            store,
            sound_engine,
            audio_out,
            memory_values,
        };

        if let Some(data_pack) = &rom.data_pack {
            let length = data_pack.data.len() as i32;

            let datapack_start = out.call_datapack(length);
            let mut datapack_end = 0;
            if datapack_start > 0 {
                datapack_end = datapack_start + length;
            } 

            let memory = &mut instance
            .get_memory(&mut out.store, WASM_MEMORY)
            .unwrap()
            .data_mut(&mut out.store)[datapack_start as usize..datapack_end as usize];

            memory.copy_from_slice(&data_pack.data);

            out.memory_values.datapack_end = datapack_end as usize;
            out.memory_values.datapack_start = datapack_start as usize;
        };

        out.call_init();

        let memory = &mut instance
        .get_memory(&mut out.store, WASM_MEMORY)
        .unwrap();

        println!("Memory size: {}pages", memory.size(&out.store));

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

        let mem = self
            .instance
            .get_memory(&mut self.store, WASM_MEMORY)
            .unwrap();

        let before_dp = mem.data(&mut self.store)[0..self.memory_values.datapack_start].to_vec();
        let after_dp = mem.data(&mut self.store)
            [self.memory_values.datapack_end..]
            .to_vec();

        let sound_engine_data = self.store.data().audio_context.sound_engine_data.clone();

        WasmConsoleState {
            previous_buttons,
            before_dp,
            after_dp,
            sound_engine_data,
        }
    }

    pub fn load_save_state(&mut self, state: WasmConsoleState) {
        let WasmConsoleState {
            previous_buttons,
            before_dp,
            after_dp,
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

        let memory = &mut self
            .instance
            .get_memory(&mut self.store, WASM_MEMORY)
            .unwrap()
            .data_mut(&mut self.store);

        memory[0..self.memory_values.datapack_start].copy_from_slice(&before_dp);
        memory[self.memory_values.datapack_end..]
            .copy_from_slice(&after_dp);
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
                if window.set_cursor_grab(CursorGrabMode::Confined).is_err()
                    && window.set_cursor_grab(CursorGrabMode::Locked).is_err()
                {
                    println!("Error: Failed to lock mouse");
                }
                window.set_cursor_visible(false);
            }
            false => {
                if let Err(e) = window.set_cursor_grab(CursorGrabMode::None) {
                    println!("{e}");
                }
                window.set_cursor_visible(true);
            }
        }
    }
}

fn call<T>(func: Option<&GameFunc>, store: &mut Store<T>) {
    if let Some(func) = func {
        func.call(store, ()).unwrap()
    }
}

impl Console for WasmConsole {
    fn call_init(&mut self) {
        call(self.functions.init_fn.as_ref(), &mut self.store);
    }

    fn call_datapack(&mut self, len: i32) -> i32 {
        if let Some(datapack) = &self.functions.datapack_fn {
            datapack.call(&mut self.store, len).unwrap()
        } else {
            0
        }
    }

    fn call_update(&mut self) {
        call(self.functions.update_fn.as_ref(), &mut self.store);
    }

    fn call_draw(&mut self) {
        call(self.functions.draw_fn.as_ref(), &mut self.store);
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.store.data().draw_context.frame_buffer.pixel_buffer);
    }

    fn handle_requests(&mut self, requests: Vec<GgrsRequest<Self>>) {
        for request in requests {
            match request {
                GgrsRequest::SaveGameState { cell, frame } => {
                    let state = self.generate_save_state();
                    cell.save(frame, Some(state), None);
                }
                GgrsRequest::LoadGameState { cell, .. } => {
                    let state = cell.load().expect("Failed to load game state");
                    self.load_save_state(state);
                }
                GgrsRequest::AdvanceFrame { inputs } => {
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
