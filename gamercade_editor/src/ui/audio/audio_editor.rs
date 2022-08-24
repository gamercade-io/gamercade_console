use eframe::egui::Ui;

use crate::editor_data::EditorSoundData;

use super::{ChainEditor, InstrumentEditor, PatternEditor, SongEditor};

#[derive(Clone, Debug)]
pub struct AudioEditor {
    pub mode: AudioEditorMode,
    chain_editor: ChainEditor,
    instrument_editor: InstrumentEditor,
    pattern_editor: PatternEditor,
    song_editor: SongEditor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioEditorMode {
    Instrument,
    Songs,
    Chains,
    Patterns,
}

impl Default for AudioEditor {
    fn default() -> Self {
        Self {
            mode: AudioEditorMode::Instrument,
            chain_editor: ChainEditor::default(),
            instrument_editor: InstrumentEditor::default(),
            pattern_editor: PatternEditor::default(),
            song_editor: SongEditor::default(),
        }
    }
}

impl AudioEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, AudioEditorMode::Instrument, "Instruments");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Songs, "Songs");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Chains, "Chains");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Patterns, "Patterns");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, _data: &mut EditorSoundData) {
        match self.mode {
            AudioEditorMode::Instrument => self.instrument_editor.draw(ui),
            AudioEditorMode::Songs => self.song_editor.draw(ui),
            AudioEditorMode::Chains => self.chain_editor.draw(ui),
            AudioEditorMode::Patterns => self.pattern_editor.draw(ui),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        //TODO: Write this
        ui.label("TODO!");
    }
}
