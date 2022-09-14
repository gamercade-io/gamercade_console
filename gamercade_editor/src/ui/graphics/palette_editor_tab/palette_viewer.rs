use eframe::egui::{Color32, ImageButton, TextureId, Ui, Vec2};
use gamercade_core::{Color, Palette, PALETTE_COLORS};

use crate::editor_data::EditorPalette;

#[derive(Clone, Default, Debug)]
pub struct PaletteViewer {
    pub(crate) selected_color: usize,
}

impl PaletteViewer {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        palette: &mut EditorPalette,
        palette_texture: TextureId,
    ) {
        ui.group(|ui| {
            ui.label("Palette Viewer");

            ui.label("Palette Name: ");
            ui.text_edit_singleline(&mut palette.name);

            let palette = &mut palette.palette;

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };

                ui.horizontal(|ui| {
                    (0..PALETTE_COLORS / 8).for_each(|x| {
                        ui.vertical(|ui| {
                            (0..8).for_each(|y| {
                                let index = x + (y * 8);
                                let selected = index == self.selected_color;
                                let color = palette.colors[index];
                                let image_button =
                                    ImageButton::new(palette_texture, Vec2 { x: 24.0, y: 24.0 })
                                        .selected(selected)
                                        .tint(Color32::from_rgba_unmultiplied(
                                            color.r, color.g, color.b, color.a,
                                        ));
                                if ui.add(image_button).clicked() {
                                    self.selected_color = index
                                };
                            });
                        });
                    })
                });
            });
        });
    }

    pub(crate) fn get_color_mut<'a>(&self, palette: &'a mut Palette) -> &'a mut Color {
        &mut palette.colors[self.selected_color]
    }
}
