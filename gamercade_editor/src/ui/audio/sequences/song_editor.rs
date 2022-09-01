use eframe::egui::Ui;
use gamercade_audio::Song;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Default)]
pub(crate) struct SongList {
    selected_sfx: usize,
}

impl AudioList<Song> for SongList {
    fn target_data(data: &EditorSoundData) -> &Vec<EditorAudioDataEntry<Song>> {
        &data.songs
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_sfx
    }

    fn name() -> &'static str {
        "Song List"
    }

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        //TODO: This
        ui.label("TODO: Draw Buttons - Song List");
    }
}

#[derive(Default)]
pub(crate) struct SongEditor {
    song_list: SongList,
}

impl SongEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        self.song_list.draw(ui, data, sync);
    }
}
