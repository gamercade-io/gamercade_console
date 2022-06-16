use crate::api::{DataApiBinding, DrawApiBinding, InputApiBinding, RandomApiBinding};

mod data_binding;
mod draw_binding;
mod input_binding;
mod random_binding;

pub fn bind_all_apis(linker: &mut wasmtime::Linker<super::Contexts>) {
    linker.bind_draw_api();
    linker.bind_input_api();
    linker.bind_random_api();
    linker.bind_data_api();
}
