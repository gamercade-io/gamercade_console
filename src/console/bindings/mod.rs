use crate::api::{GraphicsApiBinding, InputApiBinding, RandomApiBinding};

mod graphics_binding;
mod input_binding;
mod random_binding;

pub fn bind_all_apis(linker: &mut wasmtime::Linker<super::Contexts>) {
    linker.bind_graphics_api();
    linker.bind_input_api();
    linker.bind_random_api();
}
