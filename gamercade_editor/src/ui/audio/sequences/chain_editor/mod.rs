use eframe::egui::{Grid, Ui};
use gamercade_audio::Chain;

use crate::{
    editor_data::EditorSoundData,
    ui::{AudioList, AudioSyncHelper},
};

mod chain_list;
mod chain_row;

use chain_list::*;
use chain_row::*;

#[derive(Default)]
pub(crate) struct ChainEditor {
    chain_list: ChainList,

    selected_index: usize,
}

impl ChainEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.chain_list.draw(ui, data, sync);

        if let Some(chain) = &mut data.chains[self.chain_list.selected_chain].data {
            self.chain_editor_inner(ui, chain)
        };

        // TODO: Add Play & Stop Buttons

        // TODO: Add Keyboard Controls
    }

    fn chain_editor_inner(&mut self, ui: &mut Ui, chain: &mut Chain) {
        Grid::new("chain_editor_grid").striped(true).show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.spacing_mut().button_padding.x = 0.0;

            // Draw the header row
            ui.horizontal(|ui| {
                let header = ChainRow::header();
                header.draw(ui);
            });
            ui.end_row();

            // Draw the individual entries
            chain
                .entries
                .iter_mut()
                .enumerate()
                .for_each(|(row, entry)| {
                    ui.horizontal(|ui| {
                        let phrase_row = ChainRow::new(row, entry, self.selected_index);
                        if phrase_row.draw(ui) {
                            self.selected_index = row;
                        }
                    });
                    ui.end_row();
                });
        });
    }
}
