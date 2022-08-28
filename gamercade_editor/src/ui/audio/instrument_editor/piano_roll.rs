use eframe::egui::Ui;

use crate::ui::AudioSyncHelper;

#[derive(Clone, Debug, Default)]
pub struct PianoRoll {}

impl PianoRoll {
    pub(crate) fn draw(&mut self, _ui: &mut Ui, _sync: &mut AudioSyncHelper) {
        // TODO: Write this
    }
}
