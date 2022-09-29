use std::path::PathBuf;

use gamercade_core::{ButtonCode, InputState};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

use super::key_types::{Analog, AnalogAxis, AnalogDirection, AnalogSide, KeyType};

const INPUT_FILE_NAME: &str = "input.json";

impl Analog {
    pub(crate) fn adjust_input_state(self, input_state: &mut InputState) {
        let value = match self.direction {
            AnalogDirection::Positive => 1.0,
            AnalogDirection::Negative => -1.0,
        };

        let stick = match self.side {
            AnalogSide::Left => &mut input_state.left_stick,
            AnalogSide::Right => &mut input_state.right_stick,
        };

        match self.axis {
            AnalogAxis::X => stick.set_x_axis(value),
            AnalogAxis::Y => stick.set_y_axis(value),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct KeyBindings {
    pub buttons: Vec<HashMap<VirtualKeyCode, KeyType>>,
}

impl KeyBindings {
    pub fn load() -> Self {
        let path = PathBuf::from(INPUT_FILE_NAME);
        if path.exists() {
            match std::fs::read(INPUT_FILE_NAME) {
                Ok(file) => match serde_json::from_slice::<Self>(&file) {
                    Ok(key_bindings) => {
                        println!("Successfully loaded key bindings from: {}", INPUT_FILE_NAME);
                        return key_bindings;
                    }
                    Err(e) => {
                        println!("{} found, but unable to parse: {}", INPUT_FILE_NAME, e);
                    }
                },
                Err(e) => println!("{} found, but unable to read: {}", INPUT_FILE_NAME, e),
            };

            println!("Using default config.");
            Self::default()
        } else {
            println!(
                "{} not found. Generating default input file.",
                INPUT_FILE_NAME
            );
            let bindings = Self::default();

            let json = serde_json::to_string_pretty(&bindings).unwrap();

            match std::fs::write(path, json) {
                Ok(()) => println!("Successfully generated default {}", INPUT_FILE_NAME),
                Err(e) => println!("Error writing {}: {}", INPUT_FILE_NAME, e),
            };

            bindings
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let buttons = vec![[
            //Sticks
            (VirtualKeyCode::X, KeyType::Button(ButtonCode::LeftStick)),
            (VirtualKeyCode::B, KeyType::Button(ButtonCode::RightStick)),
            //Shoulders
            (VirtualKeyCode::E, KeyType::Button(ButtonCode::LeftShoulder)),
            (VirtualKeyCode::Q, KeyType::Trigger(AnalogSide::Left)),
            (
                VirtualKeyCode::R,
                KeyType::Button(ButtonCode::RightShoulder),
            ),
            (VirtualKeyCode::Y, KeyType::Trigger(AnalogSide::Right)),
            //DPad:
            (VirtualKeyCode::Up, KeyType::Button(ButtonCode::Up)),
            (VirtualKeyCode::Down, KeyType::Button(ButtonCode::Down)),
            (VirtualKeyCode::Left, KeyType::Button(ButtonCode::Left)),
            (VirtualKeyCode::Right, KeyType::Button(ButtonCode::Right)),
            //Buttons:
            (VirtualKeyCode::U, KeyType::Button(ButtonCode::A)),
            (VirtualKeyCode::I, KeyType::Button(ButtonCode::B)),
            (VirtualKeyCode::J, KeyType::Button(ButtonCode::C)),
            (VirtualKeyCode::K, KeyType::Button(ButtonCode::D)),
            (VirtualKeyCode::Key5, KeyType::Button(ButtonCode::Start)),
            (VirtualKeyCode::Key6, KeyType::Button(ButtonCode::Select)),
            //Left Stick Axis
            (
                VirtualKeyCode::W,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Positive,
                }),
            ),
            (
                VirtualKeyCode::S,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::A,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::D,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Left,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Positive,
                }),
            ),
            //Right Stick Axis,
            (
                VirtualKeyCode::T,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Positive,
                }),
            ),
            (
                VirtualKeyCode::G,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::Y,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::F,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Negative,
                }),
            ),
            (
                VirtualKeyCode::H,
                KeyType::AnalogStick(Analog {
                    side: AnalogSide::Right,
                    axis: AnalogAxis::X,
                    direction: AnalogDirection::Positive,
                }),
            ),
        ]
        .into_iter()
        .collect::<HashMap<VirtualKeyCode, KeyType>>()];

        Self { buttons }
    }
}
