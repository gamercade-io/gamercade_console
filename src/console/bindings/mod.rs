use crate::api::*;

mod audio_binding;
mod data_binding;
mod draw_binding;
mod graphics_parameter_binding;
mod input_binding;
mod multiplayer_binding;
mod random_binding;
mod text_binding;

pub fn bind_all_apis(linker: &mut wasmtime::Linker<super::Contexts>) {
    linker.bind_draw_api();
    linker.bind_input_api();
    linker.bind_random_api();
    linker.bind_data_api();
    linker.bind_graphics_parameter_api();
    linker.bind_text_api();
    linker.bind_multiplayer_api();
    linker.bind_audio_api();
}
