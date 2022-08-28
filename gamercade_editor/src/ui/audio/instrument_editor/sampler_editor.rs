use eframe::egui::Ui;
use gamercade_audio::SampleDefinition;

use crate::ui::AudioSyncHelper;

#[derive(Clone, Debug, Default)]
pub struct SamplerEditor {}

impl SamplerEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        _instrument: &mut SampleDefinition,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        ui.label("Sampler Editor: TODO");
    }
}
