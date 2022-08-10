mod algorithm;
mod feedback;
mod fm_waveform;
mod frequency_multiplier;
mod operator_definition;
mod operator_instance;
mod patch_definition;
mod patch_instance;

pub use algorithm::*;
pub use feedback::*;
pub use fm_waveform::*;
pub use frequency_multiplier::*;
pub use operator_definition::*;
pub use operator_instance::*;
pub use patch_definition::*;
pub use patch_instance::*;

pub const OPERATOR_COUNT: usize = 4;
pub const FM_OUTPUT_SAMPLE_RATE: usize = 44_100; // 44.1 khz
pub const FM_AMPLIFICATION: f32 = 25.0;
pub const LUT_QUARTER_LEN: usize = 256;
pub const LUT_FULL_LEN: usize = LUT_QUARTER_LEN * 4;
