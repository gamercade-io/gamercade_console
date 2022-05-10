use eframe::{
    egui::{ImageButton, Ui},
    epaint::{Color32, ColorImage, TextureHandle, Vec2},
};
use gamercade_core::Palette;

#[derive(Clone, Default)]
pub struct PaletteViewer {
    pub(crate) selected_color: usize,
    default_palette_texture: Option<TextureHandle>,
}

impl std::fmt::Debug for PaletteViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaletteEditor")
            .field("selected_color", &self.selected_color)
            .finish()
    }
}

impl PaletteViewer {
    pub(crate) fn draw(&mut self, ui: &mut Ui, palette: &mut Palette) {
        let default_palette_texture = self.default_palette_texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "default palette texture",
                ColorImage::from_rgba_unmultiplied([1, 1], &[255, 255, 255, 255]),
            )
        });

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

                        let image_button = ImageButton::new(
                            default_palette_texture.id(),
                            Vec2 { x: 32.0, y: 32.0 },
                        )
                        .selected(selected)
                        .tint(Color32::from_rgb(color.r, color.g, color.b));

                        if ui.add(image_button).clicked() {
                            self.selected_color = index
                        };
                    });
            });
        });
    }
}
