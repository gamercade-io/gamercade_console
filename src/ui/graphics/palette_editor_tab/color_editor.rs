use egui::{Color32, Image, Slider, TextureId, Ui, Vec2};
use gamercade_core::Color;

#[derive(Clone, Debug, Default)]
pub struct ColorEditor {
    prev_color: Color,
    pub preview: Color,
}

impl ColorEditor {
    pub fn draw(&mut self, ui: &mut Ui, current_color: &mut Color, texture_id: TextureId) {
        if self.prev_color != *current_color {
            self.preview = *current_color;
            self.prev_color = *current_color;
        }

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Color Editor");

                // TODO: clean this up

                draw_picker(ui, texture_id, "Current", false, current_color);
                draw_picker(ui, texture_id, "Preview", true, &mut self.preview);

                ui.horizontal(|ui| {
                    if ui.button("Revert").clicked() {
                        self.preview = *current_color;
                    }

                    if ui.button("Update").clicked() {
                        *current_color = self.preview;
                        self.prev_color = self.preview;
                    }
                })
            });
        });
    }
}

fn draw_picker(
    ui: &mut Ui,
    texture_id: TextureId,
    text: &'static str,
    editable: bool,
    color: &mut Color,
) {
    ui.group(|ui| {
        ui.label(text);

        // TODO: Add hex code for color

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("R");
                    ui.add_enabled(editable, Slider::new(&mut color.r, 0..=255));
                });

                ui.horizontal(|ui| {
                    ui.label("G");
                    ui.add_enabled(editable, Slider::new(&mut color.g, 0..=255));
                });

                ui.horizontal(|ui| {
                    ui.label("B");
                    ui.add_enabled(editable, Slider::new(&mut color.b, 0..=255));
                });
            });

            ui.add(
                Image::new(texture_id, Vec2 { x: 64.0, y: 64.0 })
                    .tint(Color32::from_rgb(color.r, color.g, color.b)),
            );
        });
    });
}
