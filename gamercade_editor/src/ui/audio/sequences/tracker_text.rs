use std::fmt::Debug;

use eframe::{
    egui::{Button, RichText, Ui},
    epaint::Color32,
};
use tinystr::TinyAsciiStr;

pub(crate) struct TrackerText<const N: usize> {
    text: TinyAsciiStr<N>,
    text_color: Color32,
    bg_color: Option<Color32>,
}

impl<const N: usize> TrackerText<N> {
    pub fn new(text: &str, text_color: Color32, bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_str(text).unwrap(),
            text_color,
            bg_color,
        }
    }

    pub fn draw_editable<'a, T: Debug>(&self, ui: &mut Ui, field: &mut T) -> bool {
        let mut text = RichText::new(self.text.as_str())
            .color(self.text_color)
            .monospace();
        if let Some(bg_color) = self.bg_color {
            text = text.background_color(bg_color)
        };
        ui.add(Button::new(text).frame(false)).clicked()
    }

    pub fn draw_none(&self, ui: &mut Ui) -> bool {
        let mut text = RichText::new(self.text.as_str())
            .color(self.text_color)
            .monospace();
        if let Some(bg_color) = self.bg_color {
            text = text.background_color(bg_color)
        };
        ui.add(Button::new(text).frame(false)).clicked()
    }

    pub fn separator(bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_bytes(&[u8::try_from(' ').unwrap(); N]).unwrap(),
            text_color: Color32::DARK_GRAY,
            bg_color,
        }
    }

    pub fn new_empty(bg_color: Option<Color32>) -> Self {
        Self {
            text: TinyAsciiStr::from_bytes(&[u8::try_from('-').unwrap(); N]).unwrap(),
            text_color: Color32::DARK_GRAY,
            bg_color,
        }
    }
}
