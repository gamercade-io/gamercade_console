use eframe::egui::{ScrollArea, Ui};

use crate::{editor_data::EditorSoundData, ui::AudioSyncHelper};

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

                                if ui.selectable_label(is_checked, &instrument.name).clicked() {
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
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        ui.horizontal(|ui| {
            if ui.button("New").clicked() {
                // TODO:
                println!("new instrument clicked")
            }

            if ui.button("Delete").clicked() {
                // TODO:
                println!("delete instrument clicked")
            }
        });
    }
}
