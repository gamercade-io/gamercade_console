use egui::{ComboBox, Ui};
use gilrs::Gilrs;

use crate::console::{InputMode, LocalInputManager};

#[derive(Default)]
pub struct ControllerGui {}

impl ControllerGui {
    pub(crate) fn draw(&self, ui: &mut Ui, input: &mut LocalInputManager, gilrs: &Gilrs) {
        ui.group(|ui| {
            ui.label("Controller Settings:");
            let combo_text = match input.input_mode {
                InputMode::Emulated => String::from("Keyboard"),
                InputMode::Gamepad(id) => format!("Gamepad: {}", id),
            };
            ComboBox::from_label("Select Controller")
                .selected_text(combo_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut input.input_mode, InputMode::Emulated, "Keyboard");

                    gilrs.gamepads().for_each(|(id, name)| {
                        ui.selectable_value(
                            &mut input.input_mode,
                            InputMode::Gamepad(id),
                            name.name(),
                        );
                    });
                });
        });
    }
}
