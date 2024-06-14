use eframe::egui::{Checkbox, Color32, Image, Slider, TextEdit, TextureHandle, Ui, Vec2};
use gamercade_core::Color;

#[derive(Clone, Debug, Default)]
pub struct ColorEditor {
    prev_color: Color,
    pub preview: Color,
    pub hex_preview: String,
}

impl ColorEditor {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        current_color: &mut Color,
        texture_handle: &TextureHandle,
        index: usize,
    ) {
        if self.prev_color != *current_color {
            self.preview = *current_color;
            self.prev_color = *current_color;
            self.hex_preview = (*current_color).to_hex_string();
        }

        let mut current_hex = (*current_color).to_hex_string();

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Color Editor");

                ui.label(format!("Color index: {index}"));

                draw_picker(
                    ui,
                    texture_handle,
                    "Current",
                    false,
                    current_color,
                    &mut current_hex,
                );
                draw_picker(
                    ui,
                    texture_handle,
                    "Preview",
                    true,
                    &mut self.preview,
                    &mut self.hex_preview,
                );

                ui.horizontal(|ui| {
                    if ui.button("Revert").clicked() {
                        self.preview = *current_color;
                        self.hex_preview = (*current_color).to_hex_string();
                    }

                    if ui.button("Update").clicked() {
                        *current_color = self.preview;
                        self.prev_color = self.preview;
                        self.hex_preview = current_hex;
                    }
                })
            });
        });
    }
}

fn draw_picker(
    ui: &mut Ui,
    texture_handle: &TextureHandle,
    text: &'static str,
    editable: bool,
    color: &mut Color,
    hex_color: &mut String,
) {
    ui.group(|ui| {
        ui.label(text);

        // TODO: Add hex code for color

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("R");
                    if ui
                        .add_enabled(editable, Slider::new(&mut color.r, 0..=255))
                        .changed()
                    {
                        *hex_color = color.to_hex_string();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("G");
                    if ui
                        .add_enabled(editable, Slider::new(&mut color.g, 0..=255))
                        .changed()
                    {
                        *hex_color = color.to_hex_string();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("B");
                    if ui
                        .add_enabled(editable, Slider::new(&mut color.b, 0..=255))
                        .changed()
                    {
                        *hex_color = color.to_hex_string();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Hex");
                    if ui
                        .add_enabled(
                            editable,
                            TextEdit::singleline(hex_color).desired_width(64.0),
                        )
                        .changed()
                    {
                        let _result = color.update_from_hex_string(hex_color);
                    }
                });

                ui.horizontal(|ui| {
                    let mut checked = color.a == 0xFF;
                    let checkbox = Checkbox::new(&mut checked, "Visible");
                    if ui.add_enabled(editable, checkbox).changed() {
                        if checked {
                            color.a = 0xFF;
                        } else {
                            color.a = 0;
                        }
                    };
                });
            });

            ui.add(
                Image::new(texture_handle)
                    .fit_to_exact_size(Vec2::new(64.0, 64.0))
                    .tint(Color32::from_rgba_unmultiplied(
                        color.r, color.g, color.b, color.a,
                    )),
            );
        });
    });
}
