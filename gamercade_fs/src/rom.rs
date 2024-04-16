use std::{fs, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};

use crate::{bundle, DataPack, EditorRom, GameAssetProvider, GameCodeProvider};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: GraphicsData,
    pub sounds: SoundRom,
    pub code: Box<[u8]>,
    pub data_pack: Option<DataPack>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            frame_rate: Default::default(),
            graphics: Default::default(),
            sounds: Default::default(),
            code: Default::default(),
            player_count: (1, 1),
            data_pack: Default::default(),
        }
    }
}

impl Rom {
    pub const fn height(&self) -> i32 {
        self.resolution.height()
    }

    pub const fn width(&self) -> i32 {
        self.resolution.width()
    }

    pub fn try_load_bytes(bytes: &[u8]) -> Result<Self, String> {
        let reader = zstd::Decoder::new(bytes).map_err(|e| e.to_string())?;
        Ok(bincode::deserialize_from::<_, Self>(reader).map_err(|e| e.to_string())?)
    }

    pub fn try_load(path: &PathBuf) -> Result<Self, String> {
        let file = fs::File::open(path).map_err(|e| e.to_string())?;

        match path.extension().and_then(|path| path.to_str()) {
            Some("gcrom") => {
                let reader = zstd::Decoder::new(file).map_err(|e| e.to_string())?;
                bincode::deserialize_from::<_, Self>(reader).map_err(|e| e.to_string())
            }
            Some("wasm") => {
                println!("No assets provided. Using default Asset pack.");
                let code = crate::try_load_wasm(path)?;
                let assets = EditorRom::default();
                Ok(bundle(&code, &assets))
            }
            _ => Err("Invalid file extension.".to_string()),
        }
    }

    pub fn try_save(&self, path: &PathBuf) -> Result<(), String> {
        let rom = bincode::serialize(self).map_err(|e| e.to_string())?;
        let target = fs::File::create(path).map_err(|e| e.to_string())?;
        let mut encoder = zstd::Encoder::new(target, zstd::DEFAULT_COMPRESSION_LEVEL)
            .map_err(|e| e.to_string())?;

        encoder.write_all(&rom).map_err(|e| e.to_string())?;

        encoder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl GameCodeProvider for Rom {
    fn code(&self) -> &[u8] {
        &self.code
    }
}

impl GameAssetProvider for Rom {
    fn resolution(&self) -> Resolution {
        self.resolution
    }

    fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }

    fn player_count(&self) -> (usize, usize) {
        self.player_count
    }

    fn graphics(&self) -> GraphicsData {
        self.graphics.clone()
    }

    fn sounds(&self) -> SoundRom {
        self.sounds.clone()
    }

    fn data_pack(&self) -> Option<DataPack> {
        self.data_pack.clone()
    }
}
