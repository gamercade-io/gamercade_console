// Own imports
mod palette_preview;
mod sheet_editor;
mod sheet_list;
mod sheet_settings;
mod sprite_sheet_importer;

use std::{fmt::Display, str::FromStr};

use gamercade_core::{ColorIndex, Palette, SpriteIndex, SpriteSheetIndex};
use hashbrown::HashMap;
use palette_preview::PalettePreview;
use sheet_editor::SheetEditor;
use sheet_list::SheetList;
use sheet_settings::SheetSettings;

// Externals
use eframe::egui::{TextEdit, TextureHandle, Ui};

use super::PaletteEditor;
use gamercade_fs::EditorGraphicsData;

#[derive(Clone, Default)]
pub struct SpriteSheetEditor {
    list: SheetList,
    settings: SheetSettings,
    editor: SheetEditor,
    palette_preview: PalettePreview,
}

impl SpriteSheetEditor {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorGraphicsData,
        palette_editor: &mut PaletteEditor,
        scale: f32,
        texture_handle: &TextureHandle,
    ) {
        ui.horizontal(|ui| {
            let selected_palette = palette_editor.selected_palette_mut();
            let palette = &data.palettes[*selected_palette].palette;
            self.list.draw(ui, &mut data.sprite_sheets, palette);

            let sheet = &mut data.sprite_sheets[self.list.selected_sheet.0 as usize];

            ui.vertical(|ui| {
                self.settings.draw(ui, sheet);
                self.editor
                    .draw(ui, &mut sheet.sprite_sheet, scale, palette);
                self.palette_preview
                    .draw(ui, &data.palettes, selected_palette, texture_handle);
            });
        });
    }

    pub fn selected_sheet(&self) -> SpriteSheetIndex {
        self.list.selected_sheet
    }

    pub fn selected_sprite(&self) -> SpriteIndex {
        self.editor.selected_sprite
    }
}

pub(crate) fn palette_to_map(palette: &Palette) -> HashMap<image::Rgba<u8>, ColorIndex> {
    palette
        .colors
        .iter()
        .enumerate()
        .map(|(index, color)| {
            (
                image::Rgba::<u8>([color.r, color.g, color.b, color.a]),
                ColorIndex(index as u8),
            )
        })
        .collect()
}

use std::fmt::Write;
pub(crate) fn typed_text_entry<T: FromStr + Display>(
    buffer: &mut String,
    editable: bool,
    label: &'static str,
    ui: &mut Ui,
    value: &mut T,
) {
    ui.label(label);

    buffer.clear();
    write!(buffer, "{value}").unwrap();
    let widget = TextEdit::singleline(buffer);
    let response = ui.add_enabled(editable, widget);

    if response.changed() {
        if let Ok(new_val) = buffer.parse() {
            *value = new_val
        }
    }
}
