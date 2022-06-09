mod editor;
mod graphics;
mod sounds;

pub use editor::*;
pub use graphics::*;
pub use sounds::*;

pub(crate) fn import_image_dialog(title: &str) -> Result<(image::RgbImage, String), String> {
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
        Ok(image) => image.into_rgb8(),
        Err(e) => return Err(format!("Failed to load iamge: {:?}", e)),
    };

    Ok((image, image_name))
}
