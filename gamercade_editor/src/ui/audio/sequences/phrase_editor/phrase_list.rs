use eframe::egui::Ui;
use gamercade_audio::{Phrase, PHRASES_MAX_COUNT};

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
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        ui.horizontal(|ui| {
            if ui.button("Add Phrase").clicked() {
                let curr_len = data.phrases.len();
                if curr_len < PHRASES_MAX_COUNT {
                    let name = format!("Phrase {}", curr_len + 1);
                    data.phrases.push(EditorAudioDataEntry {
                        name,
                        data: Some(Phrase::default()),
                    });
                    sync.notify_rom_changed();
                }
            }

            if ui.button("Clear Phrase").clicked() {
                data.phrases[self.selected_phrase].data = None;
                sync.notify_rom_changed();
            }
        });
    }
}
