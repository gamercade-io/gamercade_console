mod data_context;
mod draw_context;
mod graphics_parameter_context;
mod input_context;
mod multiplayer_context;
mod random_context;
mod text_context;

use std::sync::Arc;

use data_context::DataContext;
use draw_context::DrawContext;
use graphics_parameter_context::GraphicsParameterContext;
use input_context::InputContext;
use multiplayer_context::MultiplayerContext;
use random_context::RandomContext;
use text_context::TextContext;

use gamercade_core::Rom;

use super::SessionDescriptor;

pub struct Contexts {
    pub(crate) draw_context: DrawContext,
    pub(crate) input_context: InputContext,
    pub(crate) random_context: RandomContext,
    pub(crate) data_context: DataContext,
    pub(crate) graphics_parameter_context: GraphicsParameterContext,
    pub(crate) text_context: TextContext,
    pub(crate) multiplayer_context: MultiplayerContext,
}

impl Contexts {
    pub fn new(rom: &Arc<Rom>, session: SessionDescriptor) -> Self {
        Self {
            draw_context: DrawContext::new(rom.clone()),
            input_context: InputContext::new(session.num_players),
            random_context: RandomContext::new(156263),
            data_context: DataContext::new(rom.clone()),
            graphics_parameter_context: GraphicsParameterContext::default(),
            text_context: TextContext::default(),
            multiplayer_context: MultiplayerContext::new(session),
        }
    }
}
