mod audio_context;
mod data_context;
mod draw_context;
mod graphics_parameter_context;
mod input_context;
mod multiplayer_context;
mod random_context;
mod text_context;

use std::sync::Arc;

use audio_context::*;
use data_context::DataContext;
use draw_context::DrawContext;
use gamercade_fs::Rom;
use gamercade_sound_engine::SoundRomInstance;
use graphics_parameter_context::GraphicsParameterContext;
use input_context::InputContext;
use multiplayer_context::MultiplayerContext;
use random_context::RandomContext;
use text_context::TextContext;

use super::SessionDescriptor;
pub struct Contexts {
    pub(crate) draw_context: DrawContext,
    pub(crate) input_context: InputContext,
    pub(crate) random_context: RandomContext,
    pub(crate) data_context: DataContext,
    pub(crate) graphics_parameter_context: GraphicsParameterContext,
    pub(crate) text_context: TextContext,
    pub(crate) multiplayer_context: MultiplayerContext,
    pub(crate) audio_context: AudioContext,
}

impl Contexts {
    pub fn new(
        rom: &Arc<Rom>,
        seed: u64,
        session: SessionDescriptor,
        sound_rom: &Arc<SoundRomInstance>,
        output_sample_rate: usize,
    ) -> Self {
        Self {
            draw_context: DrawContext::new(rom.clone()),
            input_context: InputContext::new(session.num_players),
            random_context: RandomContext::new(seed),
            data_context: DataContext::new(rom.clone()),
            graphics_parameter_context: GraphicsParameterContext,
            text_context: TextContext,
            multiplayer_context: MultiplayerContext::new(session),
            audio_context: AudioContext::new(sound_rom, output_sample_rate),
        }
    }
}
