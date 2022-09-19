pub(crate) mod bundler;

use std::path::PathBuf;

use gamercade_fs::{EditorRom, Rom};

enum ReadFileResult {
    Rom(Rom),
    EditorRom(EditorRom),
    Code(Box<[u8]>),
}

fn read_path(path: &PathBuf) -> Result<ReadFileResult, String> {
    match path.extension().and_then(|path| path.to_str()) {
        Some("gcrom") => {
            let rom = Rom::try_load(path)?;
            Ok(ReadFileResult::Rom(rom))
        }
        Some("gce") => {
            let editor_rom = EditorRom::try_load(path)?;
            Ok(ReadFileResult::EditorRom(editor_rom))
        }
        Some("wasm") => {
            let code = gamercade_fs::try_load_wasm(path)?;
            Ok(ReadFileResult::Code(code.into_boxed_slice()))
        }
        _ => Err("Invalid file extension.".to_string()),
    }
}
