use eframe::egui::Ui;
use gamercade_audio::WavetableDefinition;

use crate::ui::AudioSyncHelper;

#[derive(Clone, Debug, Default)]
pub struct WavetableEditor {}

impl WavetableEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        _instrument: &mut WavetableDefinition,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        ui.label("Wavetable Editor: TODO");
    }
}
