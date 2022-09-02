use eframe::{
    egui::{Grid, Key, Ui},
    epaint::Color32,
};

use gamercade_audio::{InstrumentId, NoteId, Phrase, PhraseEntry, PHRASE_MAX_ENTRIES};

use super::TrackerText;
use crate::{
    editor_data::EditorSoundData,
    ui::{AudioList, AudioSyncHelper},
};

mod phrase_list;
mod phrase_row;

use phrase_list::*;
use phrase_row::*;

#[derive(Debug, Default)]
pub(crate) struct PhraseEditor {
    phrase_list: PhraseList,
    selected_entry: SelectedEntry,
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct SelectedEntry {
    index: usize,
    mode: SelectedEntryMode,
}

impl SelectedEntry {
    fn up(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    fn down(&mut self) {
        self.index += 1;
        if self.index == PHRASE_MAX_ENTRIES {
            self.index = PHRASE_MAX_ENTRIES - 1
        };
    }

    fn left(&mut self) {
        self.mode.left()
    }

    fn right(&mut self) {
        self.mode.right()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SelectedEntryMode {
    #[default]
    None,
    Note,
    Volume,
    Instrument,
}

impl SelectedEntryMode {
    fn right(&mut self) {
        match self {
            SelectedEntryMode::None => *self = SelectedEntryMode::Note,
            SelectedEntryMode::Note => *self = SelectedEntryMode::Volume,
            SelectedEntryMode::Volume => *self = SelectedEntryMode::Instrument,
            SelectedEntryMode::Instrument => *self = SelectedEntryMode::Note,
        }
    }

    fn left(&mut self) {
        match self {
            SelectedEntryMode::None => *self = SelectedEntryMode::Volume,
            SelectedEntryMode::Note => *self = SelectedEntryMode::Instrument,
            SelectedEntryMode::Volume => *self = SelectedEntryMode::Note,
            SelectedEntryMode::Instrument => *self = SelectedEntryMode::Volume,
        }
    }
}

type PhraseEntryType = PhraseEntry<NoteId, InstrumentId>;

impl PhraseEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.phrase_list.draw(ui, data, sync);

        if let Some(phrase) = &mut data.phrases[self.phrase_list.selected_phrase].data {
            self.phrase_editor_inner(ui, phrase, sync)
        }

        let input = ui.input();

        if input.key_pressed(Key::ArrowUp) {
            self.selected_entry.up()
        }

        if input.key_pressed(Key::ArrowDown) {
            self.selected_entry.down()
        }

        if input.key_pressed(Key::ArrowLeft) {
            self.selected_entry.left()
        }

        if input.key_pressed(Key::ArrowRight) {
            self.selected_entry.right()
        }
    }

    fn phrase_editor_inner(
        &mut self,
        ui: &mut Ui,
        phrase: &mut Phrase,
        sync: &mut AudioSyncHelper,
    ) {
        Grid::new("phase_editor_grid").striped(true).show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.spacing_mut().button_padding.x = 0.0;

            // Draw the header row
            ui.horizontal(|ui| {
                let header = PhraseRow {
                    row: TrackerText::new("# ", Color32::GRAY, None),
                    note: TrackerText::new("N  ", Color32::GRAY, None),
                    volume: TrackerText::new("V ", Color32::GRAY, None),
                    instrument: TrackerText::new("I ", Color32::GRAY, None),
                    separator: TrackerText::separator(None),
                };
                header.draw(ui, &mut None, sync);
            });
            ui.end_row();

            // Draw the individual entries
            phrase
                .entries
                .iter_mut()
                .enumerate()
                .for_each(|(row, entry)| {
                    ui.horizontal(|ui| {
                        let phrase_row = PhraseRow::new(row, entry, self.selected_entry);
                        if let Some(selected) = phrase_row.draw(ui, entry, sync) {
                            self.selected_entry.index = row;
                            self.selected_entry.mode = selected;
                        }
                    });
                    ui.end_row();
                });
        });
    }
}
