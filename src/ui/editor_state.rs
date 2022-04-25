use gamercade_core::Rom;

use super::GraphicsEditor;

pub struct EditorState {
    pub mode: EditorMode,
    pub rom: Rom,
}

pub enum EditorMode {
    GraphicsMode(GraphicsEditor),
    SoundMode,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            mode: EditorMode::GraphicsMode(GraphicsEditor::default()),
            rom: Rom::default(),
        }
    }
}
