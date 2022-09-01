use eframe::egui::Ui;
use gamercade_audio::Phrase;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Debug, Default)]
pub(crate) struct PhraseEditor {
    phrase_list: PhraseList,
}

#[derive(Debug, Default)]
pub(crate) struct PhraseList {
    selected_phrase: usize,
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

impl PhraseEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.phrase_list.draw(ui, data, sync);

        if let Some(phrase) = &mut data.phrases[self.phrase_list.selected_phrase].data {
            phrase_editor_inner(ui, phrase, sync)
        }
    }
}

fn phrase_editor_inner(ui: &mut Ui, phrase: &mut Phrase, _sync: &mut AudioSyncHelper) {
    ui.vertical(|_ui| {
        phrase.entries.iter().for_each(|_entry| {
            // TODO: This
        });
    });
}
