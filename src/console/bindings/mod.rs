use crate::api::{DrawApiBinding, InputApiBinding, RandomApiBinding};

mod draw_binding;
mod input_binding;
mod random_binding;

pub fn bind_all_apis(linker: &mut wasmtime::Linker<super::Contexts>) {
    linker.bind_draw_api();
    linker.bind_input_api();
    linker.bind_random_api();
}
