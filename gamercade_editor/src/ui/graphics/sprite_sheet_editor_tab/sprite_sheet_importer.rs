use eframe::egui::{Ui, Window};
use gamercade_core::{Palette, SpriteSheet};
use image::{ImageBuffer, Rgba};

use gamercade_fs::EditorSpriteSheet;

use super::{palette_to_map, typed_text_entry};

type ImageBufferBuffer = Option<(ImageBuffer<Rgba<u8>, Vec<u8>>, String)>;

#[derive(Debug, Clone, Default)]
pub(crate) struct SpriteSheetImporter {
    pub(crate) image_buffer: ImageBufferBuffer,
    text_buffer: String,
    import_mode: ImportMode,
    keep_empty_frames: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImportMode {
    RowsCols { columns: u32, rows: u32 },
    Pixels { width: u32, height: u32 },
}

impl Default for ImportMode {
    fn default() -> Self {
        Self::RowsCols {
            columns: 1,
            rows: 1,
        }
    }
}

impl SpriteSheetImporter {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut Vec<EditorSpriteSheet>,
        palette: &Palette,
    ) {
        let mut done = false;

        if let Some(image) = &mut self.image_buffer {
            let ctx = ui.ctx();
            Window::new("Sprite Sheet Importer")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Sprite Sheet: {}", image.1));

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                matches!(self.import_mode, ImportMode::RowsCols { .. }),
                                "Row & Column Count",
                            )
                            .clicked()
                        {
                            self.import_mode = ImportMode::RowsCols {
                                rows: 1,
                                columns: 1,
                            };
                        }

                        if ui
                            .selectable_label(
                                matches!(self.import_mode, ImportMode::Pixels { .. }),
                                "Frame Size in Pixels",
                            )
                            .clicked()
                        {
                            self.import_mode = ImportMode::Pixels {
                                width: 8,
                                height: 8,
                            }
                        }
                    });

                    ui.separator();

                    match &mut self.import_mode {
                        ImportMode::RowsCols { rows, columns } => {
                            typed_text_entry(&mut self.text_buffer, true, "Columns", ui, columns);
                            typed_text_entry(&mut self.text_buffer, true, "Rows", ui, rows);
                        }
                        ImportMode::Pixels { width, height } => {
                            typed_text_entry(&mut self.text_buffer, true, "Width", ui, width);
                            typed_text_entry(&mut self.text_buffer, true, "Height", ui, height);
                        }
                    }

                    ui.separator();

                    ui.checkbox(&mut self.keep_empty_frames, "Keep Empty Frames");

                    ui.horizontal(|ui| {
                        if ui.button("Import Sprite Sheet").clicked() {
                            match try_import_sprite_sheet(
                                &image.0,
                                palette,
                                self.import_mode,
                                self.keep_empty_frames,
                            ) {
                                Ok(new_sheet) => {
                                    data.push(EditorSpriteSheet {
                                        name: image.1.clone(),
                                        sprite_sheet: new_sheet,
                                    });
                                    done = true;
                                }
                                Err(e) => println!("{}", e),
                            }
                        };

                        if ui.button("Cancel").clicked() {
                            done = true;
                        }
                    })
                });
        }

        if done {
            self.image_buffer = None;
        }
    }
}

struct SheetDefinition {
    width: u32,
    height: u32,
    rows: u32,
    columns: u32,
}

impl SheetDefinition {
    fn total_width(&self) -> usize {
        (self.columns * self.width) as usize
    }

    fn total_height(&self) -> usize {
        (self.rows * self.height) as usize
    }
}

fn try_import_sprite_sheet(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    palette: &Palette,
    import_mode: ImportMode,
    keep_empty_frames: bool,
) -> Result<SpriteSheet, &'static str> {
    let definition = match import_mode {
        ImportMode::RowsCols { rows, columns } => SheetDefinition {
            width: image.width() / columns,
            height: image.height() / rows,
            rows,
            columns,
        },
        ImportMode::Pixels { width, height } => SheetDefinition {
            width,
            height,
            rows: image.height() / height,
            columns: image.width() / width,
        },
    };

    let total_width = definition.total_width();
    let total_height = definition.total_height();

    if total_width != image.width() as usize || total_height != image.height() as usize {
        return Err("Invalid dimensions. Check that sprite sheet dimensions are evenly divisible");
    }

    let colors = palette_to_map(palette);
    let no_alpha_color_index = colors
        .iter()
        .find(|(color, _)| color.0[3] == 0)
        .map(|(_, index)| *index);

    let mut final_output = Vec::with_capacity(total_width * total_height);
    let mut frame = Vec::with_capacity((definition.width * definition.height) as usize);
    let mut frame_count = 0;

    for row in 0..definition.rows {
        let row_offset = row * definition.height;
        for column in 0..definition.columns {
            let column_offset = column * definition.width;

            // Generate the individual frame
            let mut has_colors = false;
            for y in 0..definition.height {
                for x in 0..definition.width {
                    let color = image.get_pixel(x + column_offset, y + row_offset);
                    if let (Some(no_alpha_color), 0) = (no_alpha_color_index, color.0[3]) {
                        frame.push(no_alpha_color);
                    } else if let Some(index) = colors.get(color) {
                        has_colors = true;
                        frame.push(*index);
                    } else {
                        return Err("Image contains a color not found in the palette.");
                    }
                }
            }

            // We only want to add sprites which have actual colors in them,
            // and can remove any which are completely transparent
            if has_colors || keep_empty_frames {
                frame_count += 1;

                if frame_count > u8::MAX as usize {
                    return Err("Sprite Sheet can only have 256 entries.");
                }

                final_output.append(&mut frame);
            } else {
                frame.clear()
            }
        }
    }

    Ok(SpriteSheet {
        height: definition.height as usize,
        width: definition.width as usize,
        sprites: final_output.into_boxed_slice(),
        count: frame_count as u8,
    })
}
