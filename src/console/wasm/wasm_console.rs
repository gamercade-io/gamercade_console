use std::sync::Arc;

use ggrs::GGRSRequest;
use parking_lot::Mutex;
use wasmer::{Exports, Function, ImportObject, Instance, Module, NativeFunc, Store};

use crate::{
    api::{GraphicsApiBinding, InputApiBinding},
    console::{GraphicsContext, InputContext},
    core::{InputState, PlayerInputEntry, Rom},
    Console,
};

pub struct WasmConsole {
    rom: Arc<Rom>,
    input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
    frame_buffer: Arc<Mutex<Box<[u8]>>>,
    functions: Functions,
    pub(crate) store: Store,
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) input_context: InputContext,
}

#[derive(Clone)]
pub(crate) struct Functions {
    init_fn: NativeFunc,
    update_fn: NativeFunc,
    draw_fn: NativeFunc,
}

impl Functions {
    fn find_functions(instance: &Instance) -> Self {
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
            frame_buffer: frame_buffer.clone(),
            rom: rom.clone(),
        };
        let input_context = InputContext {
            input_entries: input_entries.clone(),
        };
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

        Self {
            store,
            rom,
            input_entries,
            frame_buffer,
            functions,
            graphics_context,
            input_context,
        }
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
        buffer.copy_from_slice(&self.frame_buffer.lock());
    }

    fn handle_requests(&mut self, requests: Vec<GGRSRequest<Self>>) {
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => {
                    // TODO: This
                }
                GGRSRequest::LoadGameState { cell, frame } => {
                    //TODO: This
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
