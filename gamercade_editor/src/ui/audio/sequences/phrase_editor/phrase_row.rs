use eframe::{egui::Ui, epaint::Color32};

use crate::ui::audio::sequences::TrackerText;

use super::{PhraseEntryType, SelectedEntry, SelectedEntryMode};

pub(crate) struct PhraseRow {
    pub(crate) row: TrackerText<3>,
    pub(crate) note: TrackerText<3>,
    pub(crate) volume: TrackerText<2>,
    pub(crate) instrument: TrackerText<2>,
    pub(crate) separator: TrackerText<2>,
}

const DEFAULT_TEXT_COLOR: Color32 = Color32::GRAY;
const SELECTED_BG_COLOR: Color32 = Color32::DARK_BLUE;
const EDITING_BG_COLOR: Color32 = Color32::LIGHT_BLUE;

impl PhraseRow {
    pub(crate) fn new(
        row: usize,
        entry: &Option<PhraseEntryType>,
        selected: SelectedEntry,
    ) -> Self {
        let bg_color = if selected.index == row {
            Some(SELECTED_BG_COLOR)
        } else {
            None
        };

        let row = TrackerText::new(&format!("{:X}:", row), DEFAULT_TEXT_COLOR, bg_color);
        let separator = TrackerText::separator(bg_color);

        if let Some(entry) = entry {
            PhraseRow {
                row,
                note: TrackerText::new(
                    &gamercade_audio::get_note(entry.note).name,
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Note && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                volume: TrackerText::new(
                    &format!("{:02X}", entry.volume),
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Volume && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                instrument: TrackerText::new(
                    &format!("{:02X}", entry.instrument.0),
                    DEFAULT_TEXT_COLOR,
                    if selected.mode == SelectedEntryMode::Instrument && bg_color.is_some() {
                        Some(EDITING_BG_COLOR)
                    } else {
                        bg_color
                    },
                ),
                separator,
            }
        } else {
            PhraseRow {
                row,
                note: TrackerText::new_empty(bg_color),
                volume: TrackerText::new_empty(bg_color),
                instrument: TrackerText::new_empty(bg_color),
                separator,
            }
        }
    }

    pub(crate) fn draw(&self, ui: &mut Ui) -> Option<SelectedEntryMode> {
        let results = [
            self.row.draw(ui),
            self.separator.draw(ui),
            self.note.draw(ui),
            self.separator.draw(ui),
            self.volume.draw(ui),
            self.separator.draw(ui),
            self.instrument.draw(ui),
        ];

        if let Some((index, _)) = results.into_iter().enumerate().find(|(_, result)| *result) {
            match index {
                2 => Some(SelectedEntryMode::Note),
                4 => Some(SelectedEntryMode::Volume),
                6 => Some(SelectedEntryMode::Instrument),
                _ => Some(SelectedEntryMode::None),
            }
        } else {
            None
        }
    }
}
