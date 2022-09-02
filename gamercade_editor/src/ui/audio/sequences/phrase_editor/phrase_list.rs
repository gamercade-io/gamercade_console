use eframe::egui::Ui;
use gamercade_audio::Phrase;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Debug, Default)]
pub(crate) struct PhraseList {
    pub(crate) selected_phrase: usize,
}

impl AudioList<Option<Phrase>> for PhraseList {
    fn target_data(data: &EditorSoundData) -> &Vec<EditorAudioDataEntry<Option<Phrase>>> {
        &data.phrases
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_phrase
    }

    fn name() -> &'static str {
        "Phrase List"
    }

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        // TODO
        ui.label("TODO: Draw Buttons - Audio List");
    }
}
