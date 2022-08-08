mod algorithm;
mod feedback;
mod frequency_multiplier;
mod lut;
mod operator_definition;
mod patch_definition;
mod patch_instance;

pub use algorithm::*;
pub use feedback::*;
pub use frequency_multiplier::*;
pub use lut::*;
pub use operator_definition::*;
pub use patch_definition::*;
pub use patch_instance::*;

pub const OPERATOR_COUNT: usize = 4;
