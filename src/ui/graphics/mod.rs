mod graphics_editor;
mod palette_editor_tab;
mod sprite_editor_tab;
mod sprite_sheet_editor_tab;

pub use graphics_editor::*;
pub use palette_editor_tab::*;
pub use sprite_editor_tab::*;
pub use sprite_sheet_editor_tab::*;

pub(crate) fn import_image_dialog(title: &str) -> Result<(image::RgbaImage, String), String> {
    let path = match rfd::FileDialog::new()
        .set_title(title)
        .set_directory("/")
        .add_filter(
            "image (.png, .jpeg, .gif, .bmp, .ico, .tiff, .tga)",
            &["png", "jpeg", "gif", "bmp", "ico", "tiff", "tga"],
        )
        .pick_file()
    {
        Some(path) => path,
        None => return Err("No image selected.".to_string()),
    };

    let image_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let image = match image::open(path) {
        Ok(image) => image.into_rgba8(),
        Err(e) => return Err(format!("Failed to load iamge: {:?}", e)),
    };

    Ok((image, image_name))
}

pub(crate) fn load_buffered_image<'a>(
    ui: &mut egui::Ui,
    handle: &'a mut Option<egui::TextureHandle>,
    label: &'a str,
    rgb: egui::ColorImage,
) -> &'a egui::TextureHandle {
    match handle {
        Some(handle) => {
            handle.set(rgb);
            handle
        }
        None => {
            *handle = Some(ui.ctx().load_texture(label, rgb));
            handle.as_ref().unwrap()
        }
    }
}
