use gamercade_audio::SoundRom;
use gamercade_core::{FrameRate, GraphicsData, Resolution};

use crate::{DataPack, Rom};

/// Provides .wasm game code to produce a game Rom
pub trait GameCodeProvider {
    fn code(&self) -> &[u8];
}

impl<T> GameCodeProvider for T
where
    T: AsRef<[u8]>,
{
    fn code(&self) -> &[u8] {
        self.as_ref()
    }
}

/// Provides game assets to produce a game Rom
pub trait GameAssetProvider {
    fn resolution(&self) -> Resolution;
    fn frame_rate(&self) -> FrameRate;
    fn player_count(&self) -> (usize, usize);
    fn graphics(&self) -> GraphicsData;
    fn sounds(&self) -> SoundRom;
    fn data_pack(&self) -> Option<DataPack>;
}

/// Generates a ready-to-use Rom.
pub fn bundle<C, A>(code_provider: &C, asset_provider: &A) -> Rom
where
    C: GameCodeProvider,
    A: GameAssetProvider,
{
    Rom {
        resolution: asset_provider.resolution(),
        frame_rate: asset_provider.frame_rate(),
        player_count: asset_provider.player_count(),
        graphics: asset_provider.graphics(),
        sounds: asset_provider.sounds(),
        code: code_provider.code().into(),
        data_pack: asset_provider.data_pack(),
    }
}
