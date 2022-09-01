use eframe::egui::{ScrollArea, Ui};

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::AudioSyncHelper,
};

#[derive(Clone, Debug, Default)]
pub struct InstrumentList {
    pub selected_instrument: usize,
}

impl InstrumentList {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        ui.vertical(|ui| {
            ui.label("Instrument List:");

            // Draws the list of instruments
            ui.group(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    data.instruments
                        .iter()
                        .enumerate()
                        .for_each(|(index, instrument)| {
                            ui.horizontal(|ui| {
                                let is_checked = self.selected_instrument == index;

                                if ui
                                    .selectable_label(
                                        is_checked,
                                        format!("[{}]: {}", index, &instrument.name),
                                    )
                                    .clicked()
                                {
                                    self.selected_instrument = index
                                };
                            });
                        });
                })
            });

            self.draw_buttons(ui, data, sync);
        });
    }

    pub(crate) fn draw_buttons(
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
}
