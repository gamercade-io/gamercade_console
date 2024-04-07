mod bundler;
mod data_pack;
mod editor_data;
mod rom;

pub use bundler::*;
pub use data_pack::*;
pub use editor_data::*;
pub use rom::*;

pub fn try_load_wasm(path: &std::path::PathBuf) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| e.to_string())
}
