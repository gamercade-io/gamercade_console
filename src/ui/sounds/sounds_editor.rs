use eframe::egui::Ui;

use crate::editor_data::EditorSoundsData;

use super::{PatchEditor, SequenceEditor};

#[derive(Clone, Debug)]
pub struct SoundsEditor {
    pub mode: SoundsEditorMode,
    patch_editor: PatchEditor,
    sequence_editor: SequenceEditor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SoundsEditorMode {
    Patches,
    Sequences,
}

impl Default for SoundsEditor {
    fn default() -> Self {
        Self {
            mode: SoundsEditorMode::Patches,
            patch_editor: Default::default(),
            sequence_editor: Default::default(),
        }
    }
}

impl SoundsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, SoundsEditorMode::Patches, "Patches");
        ui.selectable_value(&mut self.mode, SoundsEditorMode::Sequences, "Sequences");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, _data: &mut EditorSoundsData) {
        match self.mode {
            SoundsEditorMode::Patches => self.patch_editor.draw(ui),
            SoundsEditorMode::Sequences => self.sequence_editor.draw(ui),
        };
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        //TODO: Write this
        ui.label("TODO!");
    }
}
