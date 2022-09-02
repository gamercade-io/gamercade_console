use eframe::egui::Ui;

use crate::{
    editor_data::EditorSoundData,
    ui::{AudioList, AudioSyncHelper},
};

mod chain_list;
use chain_list::*;

#[derive(Default)]
pub(crate) struct ChainEditor {
    chain_list: ChainList,
}

impl ChainEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        self.chain_list.draw(ui, data, sync);
    }
}
