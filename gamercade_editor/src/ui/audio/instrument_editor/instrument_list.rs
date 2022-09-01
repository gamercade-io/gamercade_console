use eframe::egui::Ui;
use gamercade_audio::InstrumentDataDefinition;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Default)]
pub struct InstrumentList {
    pub selected_instrument: usize,
}

impl AudioList<Option<InstrumentDataDefinition>> for InstrumentList {
    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        ui.horizontal(|ui| {
            if ui.button("New").clicked() {
                data.instruments.push(EditorAudioDataEntry::default());
                sync.notify_rom_changed()
            }

            if ui.button("Clear Instrument").clicked() {
                data.instruments[self.selected_instrument] = EditorAudioDataEntry::default();
                sync.notify_rom_changed()
            }

            if ui.button("Clean Up Instruments").clicked() {
                // TODO: Clean up unused instruments
                // Have to iterate through the list and find the highest "non-none" value
                // Then we can remove all of those after it.
                println!("Clean Up Instruments")
            }
        });
    }

    fn target_data(
        data: &EditorSoundData,
    ) -> &Vec<EditorAudioDataEntry<Option<InstrumentDataDefinition>>> {
        &data.instruments
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_instrument
    }

    fn name() -> &'static str {
        "Instrument List"
    }
}
