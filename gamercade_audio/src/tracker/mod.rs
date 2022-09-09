mod chain;
mod effect;
mod phrase;
mod song;

pub use chain::*;
pub use effect::*;
pub use phrase::*;
pub use song::*;

pub type PhraseVolumeType = u8;

pub fn to_scaled_value(volume: PhraseVolumeType) -> f32 {
    let val = (volume as f32).powi(2);
    let max = (PhraseVolumeType::MAX as f32).powi(2);
    val / max
}
