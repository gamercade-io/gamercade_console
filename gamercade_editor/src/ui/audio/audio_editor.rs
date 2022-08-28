use std::sync::Arc;

use eframe::egui::Ui;
use gamercade_sound_engine::{SoundEngine, SoundEngineData, SoundRomInstance};

use crate::editor_data::EditorSoundData;

use super::{ChainEditor, InstrumentEditor, PatternEditor, SfxEditor, SongEditor};

pub struct AudioEditor {
    pub mode: AudioEditorMode,
    chain_editor: ChainEditor,
    instrument_editor: InstrumentEditor,
    pattern_editor: PatternEditor,
    song_editor: SongEditor,
    sfx_editor: SfxEditor,

    sound_engine: SoundEngine,
    audio_sync_helper: AudioSyncHelper,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioEditorMode {
    Instrument,
    Sfx,
    Songs,
    Chains,
    Patterns,
}

impl AudioEditor {
    pub(crate) fn new(data: &EditorSoundData) -> Self {
        let sound_rom_instance = Arc::new(SoundRomInstance::from(data));
        let sound_engine = SoundEngine::new(60, &sound_rom_instance, 0);

        let sound_engine_data =
            SoundEngineData::new(sound_engine.output_sample_rate(), &sound_rom_instance);

        Self {
            mode: AudioEditorMode::Instrument,
            chain_editor: ChainEditor::default(),
            instrument_editor: InstrumentEditor::default(),
            pattern_editor: PatternEditor::default(),
            song_editor: SongEditor::default(),
            sfx_editor: SfxEditor::default(),
            sound_engine,
            audio_sync_helper: AudioSyncHelper {
                should_sync: false,
                sound_engine_data,
            },
        }
    }
}

pub(crate) struct AudioSyncHelper {
    should_sync: bool,
    sound_engine_data: SoundEngineData,
}

impl AudioSyncHelper {
    pub(crate) fn notify(&mut self) {
        self.should_sync = true;
    }

    pub(crate) fn try_sync(&mut self, data: &EditorSoundData, sound_engine: &mut SoundEngine) {
        if self.should_sync {
            let new_instance = Arc::new(SoundRomInstance::from(data));
            self.sound_engine_data
                .replace_sound_rom_instance(&new_instance);
            sound_engine.sync_audio_thread(&self.sound_engine_data);
            self.should_sync = false;
        }
    }
}

impl AudioEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, AudioEditorMode::Instrument, "Instruments");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Patterns, "Patterns");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Chains, "Chains");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Songs, "Songs");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Sfx, "Sfx");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, data: &mut EditorSoundData) {
        match self.mode {
            AudioEditorMode::Instrument => {
                self.instrument_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Sfx => self.sfx_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Songs => self.song_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Chains => {
                self.chain_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Patterns => {
                self.pattern_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
        };

        self.audio_sync_helper
            .try_sync(data, &mut self.sound_engine);
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        //TODO: Write this
        ui.label("TODO!");
    }
}
