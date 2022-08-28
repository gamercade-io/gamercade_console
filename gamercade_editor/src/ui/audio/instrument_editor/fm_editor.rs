use eframe::egui::Ui;
use gamercade_audio::PatchDefinition;

use crate::ui::AudioSyncHelper;

#[derive(Clone, Debug, Default)]
pub struct FMEditor {}

impl FMEditor {
    pub(crate) fn draw(
        &mut self,
        _ui: &mut Ui,
        _instrument: &mut PatchDefinition,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
    }
}
