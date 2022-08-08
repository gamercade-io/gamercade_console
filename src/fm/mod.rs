mod algorithm;
mod feedback;
mod frequency_multiplier;
mod operator_definition;
mod patch_definition;

pub use algorithm::*;
pub use feedback::*;
pub use frequency_multiplier::*;
pub use operator_definition::*;
pub use patch_definition::*;

pub const OPERATOR_COUNT: usize = 4;
