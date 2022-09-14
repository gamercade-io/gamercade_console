#![deny(missing_docs)]
//! gamercade_rs is a safe wrapper over the Gamercade Wasm Api.
//!
//! The raw Api is filled with lots of unsafe calls and representation of
//! various values as primitive types such as i32 containing boolean values.
//!
//! The best way to get started is by importing the prelude via `use gamercade_rs::prelude::*`
//!
//! You must only use these functions inside of the callbacks required by the Gamercade console.
//! These functions are `init()`, `update()` and `draw()`. Calling these functions outside of the
//! designated callbacks will likely cause a panic.
//!
//! Learn more about the Gamercade Api from <https://gamercade.io/docs/api-reference>

/// The raw, unsafe Gamerade Api. It is recommended to only
/// use this in extreme circumstances.
pub mod raw;

/// The safe wrapper around the Gamercade Api, organized by primary function.
pub mod api;

/// Re-exports the entire Api for convenience, that can be glob imported via `use gamercade_rs::prelude::*`.
pub mod prelude {
    pub use crate::api::audio::*;
    pub use crate::api::data::*;
    pub use crate::api::draw::*;
    pub use crate::api::input::*;
    pub use crate::api::multiplayer::*;
    pub use crate::api::random::*;
    pub use crate::api::text::*;
    pub use gamercade_core::GraphicsParameters;
}

/// A useful struct for converting to/from Graphics parameters which are used
/// throughout draw functions.
pub use gamercade_core::GraphicsParameters;
