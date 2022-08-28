use eframe::egui::Ui;
use gamercade_audio::EnvelopeDefinition;

use crate::ui::AudioSyncHelper;

pub(crate) struct EnvelopeWidget {}

impl EnvelopeWidget {
    pub(crate) fn draw(
        ui: &mut Ui,
        _envelope: &mut EnvelopeDefinition,
        _sync: &mut AudioSyncHelper,
    ) {
        //TODO: This
        ui.label("TODO: Envelope Widget");
    }
}
