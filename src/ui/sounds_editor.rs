use eframe::egui::Ui;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoundsEditor {
    pub mode: SoundsEditorMode,
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
        }
    }
}

impl SoundsEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, SoundsEditorMode::Patches, "Patches");
        ui.selectable_value(&mut self.mode, SoundsEditorMode::Sequences, "Sequences");
    }

    pub fn draw_contents(&mut self, ui: &mut Ui) {
        match self.mode {
            SoundsEditorMode::Patches => self.patch_editor(ui),
            SoundsEditorMode::Sequences => self.sequence_editor(ui),
        };
    }

    pub fn patch_editor(&mut self, ui: &mut Ui) {
        //TODO: render the patch editor
    }

    pub fn sequence_editor(&mut self, ui: &mut Ui) {
        //TODO: render the sequence editor
    }
}
