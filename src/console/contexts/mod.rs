mod draw_context;
mod input_context;
mod random_context;

use std::sync::Arc;

use draw_context::DrawContext;
use input_context::InputContext;
use random_context::RandomContext;

use gamercade_core::Rom;

pub struct Contexts {
    pub(crate) draw_context: DrawContext,
    pub(crate) input_context: InputContext,
    pub(crate) random_context: RandomContext,
}

impl Contexts {
    pub fn new(rom: Arc<Rom>, num_players: usize) -> Self {
        Self {
            draw_context: DrawContext::new(rom),
            input_context: InputContext::new(num_players),
            random_context: RandomContext::new(156263),
        }
    }
}
