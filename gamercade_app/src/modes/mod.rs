mod arcade_mode;
mod library_mode;
mod settings_mode;

pub use arcade_mode::*;
pub use library_mode::*;
pub use settings_mode::*;

#[derive(PartialEq, Default)]
pub enum AppMode {
    #[default]
    Arcade,
    Library,
    Settings,
}
