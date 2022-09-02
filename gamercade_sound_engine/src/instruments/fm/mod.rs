mod fm_waveform;
mod operator_instance;
mod patch_instance;

pub use fm_waveform::*;
pub use operator_instance::*;
pub use patch_instance::*;

pub(crate) const LUT_QUARTER_LEN: usize = 256;
pub(crate) const LUT_FULL_LEN: usize = LUT_QUARTER_LEN * 4;
pub(crate) const FM_MODULATION: f32 = 5.0;
