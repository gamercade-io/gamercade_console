mod graphics_context;
mod input_context;
mod random_context;

use std::sync::Arc;

use graphics_context::GraphicsContext;
use input_context::InputContext;
use random_context::RandomContext;

use crate::core::Rom;

pub struct Contexts {
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) input_context: InputContext,
    pub(crate) random_context: RandomContext,
}

impl Contexts {
    pub fn new(rom: Arc<Rom>, num_players: usize) -> Self {
        Self {
            graphics_context: GraphicsContext::new(rom),
            input_context: InputContext::new(num_players),
            random_context: RandomContext {},
        }
    }
}
