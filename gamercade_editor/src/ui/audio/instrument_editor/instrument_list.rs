use eframe::egui::Ui;

use crate::{editor_data::EditorSoundData, ui::AudioSyncHelper};

#[derive(Clone, Debug, Default)]
pub struct InstrumentList {
    pub selected_instrument: usize,
}

impl InstrumentList {
    pub(crate) fn draw(
        &mut self,
        _ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
    }
}
