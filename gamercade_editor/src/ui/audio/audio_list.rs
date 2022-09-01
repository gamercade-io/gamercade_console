use eframe::egui::{ScrollArea, Ui};

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::AudioSyncHelper,
};

pub(crate) trait AudioList<T> {
    fn target_data(data: &EditorSoundData) -> &Vec<EditorAudioDataEntry<T>>;
    fn selected_index(&mut self) -> &mut usize;
    fn name() -> &'static str;
    fn draw_buttons(&mut self, ui: &mut Ui, data: &mut EditorSoundData, sync: &mut AudioSyncHelper);

    fn draw(&mut self, ui: &mut Ui, data: &mut EditorSoundData, sync: &mut AudioSyncHelper) {
        ui.vertical(|ui| {
            ui.label(Self::name());

            // Draws the list of instruments
            ui.group(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    Self::target_data(data)
                        .iter()
                        .enumerate()
                        .for_each(|(index, thing)| {
                            ui.horizontal(|ui| {
                                let is_checked = *self.selected_index() == index;

                                if ui
                                    .selectable_label(
                                        is_checked,
                                        format!("[{}]: {}", index, &thing.name),
                                    )
                                    .clicked()
                                {
                                    *self.selected_index() = index
                                };
                            });
                        });
                })
            });

            self.draw_buttons(ui, data, sync);
        });
    }
}
