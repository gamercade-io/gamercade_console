use eframe::egui::Ui;
use gamercade_audio::Chain;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Default)]
pub(crate) struct ChainList {
    selected_sfx: usize,
}

impl AudioList<Option<Chain>> for ChainList {
    fn target_data(data: &EditorSoundData) -> &Vec<EditorAudioDataEntry<Option<Chain>>> {
        &data.chains
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_sfx
    }

    fn name() -> &'static str {
        "Chain List"
    }

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        //TODO: This
        ui.label("TODO: Draw Buttons - Chain List");
    }
}

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
