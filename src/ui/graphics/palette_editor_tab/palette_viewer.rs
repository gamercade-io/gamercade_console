use eframe::egui::{Color32, ImageButton, TextureId, Ui, Vec2};
use gamercade_core::{Color, Palette};

#[derive(Clone, Default, Debug)]
pub struct PaletteViewer {
    pub(crate) selected_color: usize,
}

impl PaletteViewer {
    pub(crate) fn draw(&mut self, ui: &mut Ui, palette: &mut Palette, palette_texture: TextureId) {
        ui.group(|ui| {
            ui.label("Palette Viewer");

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2 { x: 0.0, y: 0.0 };

                palette
                    .colors
                    .iter()
                    .enumerate()
                    .for_each(|(index, color)| {
                        let selected = index == self.selected_color;

                        let image_button =
                            ImageButton::new(palette_texture, Vec2 { x: 32.0, y: 32.0 })
                                .selected(selected)
                                .tint(Color32::from_rgb(color.r, color.g, color.b));

                        if ui.add(image_button).clicked() {
                            self.selected_color = index
                        };
                    });
            });
        });
    }

    pub(crate) fn get_color_mut<'a>(&self, palette: &'a mut Palette) -> &'a mut Color {
        &mut palette.colors[self.selected_color]
    }
}
