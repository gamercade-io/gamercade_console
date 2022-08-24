mod algorithm;
mod feedback;
mod fm_waveform;
mod frequency_multiplier;
mod operator_definition;
#[cfg(feature = "playback")]
mod operator_instance;
mod patch_definition;
#[cfg(feature = "playback")]
mod patch_instance;

pub use algorithm::*;
pub use feedback::*;
pub use fm_waveform::*;
pub use frequency_multiplier::*;
pub use operator_definition::*;
#[cfg(feature = "playback")]
pub use operator_instance::*;
pub use patch_definition::*;
#[cfg(feature = "playback")]
pub use patch_instance::*;

pub(crate) const OPERATOR_COUNT: usize = 4;
#[cfg(feature = "playback")]
pub(crate) const FM_AMPLIFICATION: f32 = 25.0;
pub(crate) const LUT_QUARTER_LEN: usize = 256;
pub(crate) const LUT_FULL_LEN: usize = LUT_QUARTER_LEN * 4;
