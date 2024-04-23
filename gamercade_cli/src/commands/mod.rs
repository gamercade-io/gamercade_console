pub(crate) mod bundler;
pub(crate) mod console;

use std::path::PathBuf;

use gamercade_fs::{bundle, DataPack, EditorRom, Rom};

enum ReadFileResult {
    Rom(Rom),
    EditorRom(EditorRom),
    Code(Box<[u8]>),
}

impl ReadFileResult {
    pub(crate) fn set_data_pack(&mut self, data_pack: DataPack) {
        match self {
            ReadFileResult::Rom(rom) => rom.data_pack = Some(data_pack),
            ReadFileResult::EditorRom(editor_rom) => editor_rom.data_pack = Some(data_pack),
            ReadFileResult::Code(_) => println!("Can't set a datapack for a .wasm code fiel"),
        }
    }
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

fn try_bundle_files(code: &ReadFileResult, assets: &ReadFileResult) -> Result<Rom, String> {
    match (&code, &assets) {
        (ReadFileResult::Rom(rom1), ReadFileResult::Rom(rom2)) => Ok(bundle(rom1, rom2)),
        (ReadFileResult::Rom(rom), ReadFileResult::EditorRom(editor_rom)) => {
            Ok(bundle(rom, editor_rom))
        }
        (ReadFileResult::Code(code), ReadFileResult::Rom(rom)) => Ok(bundle(code, rom)),
        (ReadFileResult::Code(code), ReadFileResult::EditorRom(editor_rom)) => {
            Ok(bundle(code, editor_rom))
        }
        (ReadFileResult::EditorRom(..), _) => {
            Err("Code provider must be a .wasm or .gcrom".to_string())
        }
        (_, ReadFileResult::Code(..)) => Err("Asset provider must be a .gce or .gcrom".to_string()),
    }
}
