use std::{fs, io::Read, net::SocketAddr, path::PathBuf};

use egui::{Button, ComboBox, Context, Slider};

use gamercade_fs::Rom;
use ggrs::{P2PSession, PlayerType, SessionBuilder, SessionState, UdpNonBlockingSocket};
use gilrs::Gilrs;
use pixels::Pixels;
use rfd::FileDialog;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    console::{InputMode, LocalInputManager, SessionDescriptor, WasmConsole, WasmConsoleState},
    DEFAULT_WINDOW_RESOLUTION,
};

pub mod framework;

pub struct Gui {
    pub window_open: bool,
    pub game_file: Option<PathBuf>,
    pub play_mode: PlayMode,
    pub remote_addr: String,
    pub player_num: usize,
    pub port: String,
    pub seed: String,

    pub wasm_console: Option<WasmConsole>,
    pub initial_state: Option<WasmConsoleState>,
}

const DEFAULT_SEED: &str = "a12cade";

impl Default for Gui {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED.to_string(),
            window_open: true,
            game_file: None,
            play_mode: PlayMode::SinglePlayer,
            remote_addr: String::new(),
            player_num: 1,
            port: String::new(),
            wasm_console: None,
            initial_state: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayMode {
    SinglePlayer,
    Networked,
}

impl Gui {
    fn ui(
        &mut self,
        pixels: &mut Pixels,
        window: &Window,
        session: &mut Option<P2PSession<WasmConsole>>,
        ctx: &Context,
        input: &mut LocalInputManager,
        gilrs: &mut Gilrs,
    ) {
        let mut is_open = self.window_open;
        egui::Window::new("Main Menu")
            .open(&mut is_open)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Select Game").clicked() {
                            self.game_file = FileDialog::new()
                                .add_filter("gcrom (.gcrom)", &["gcrom"])
                                .pick_file();
                        };

                        if let Some(file) = &self.game_file {
                            let filename = file
                                .file_name()
                                .expect("filename not found")
                                .to_string_lossy()
                                .to_string();
                            ui.label(filename);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Random Seed:");
                        ui.text_edit_singleline(&mut self.seed);
                        if u64::from_str_radix(&self.seed, 16).is_err() {
                            self.seed = DEFAULT_SEED.to_string()
                        }
                    })
                });

                ui.group(|ui| {
                    ui.label("Controller Settings:");
                    let combo_text = match input.input_mode {
                        InputMode::Emulated => String::from("Keyboard"),
                        InputMode::Gamepad(id) => format!("Gamepad: {}", id),
                    };
                    ComboBox::from_label("Select Controller")
                        .selected_text(combo_text)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut input.input_mode,
                                InputMode::Emulated,
                                "Keyboard",
                            );

                            gilrs.gamepads().for_each(|(id, name)| {
                                ui.selectable_value(
                                    &mut input.input_mode,
                                    InputMode::Gamepad(id),
                                    name.name(),
                                );
                            });
                        });
                });

                ui.group(|ui| {
                    ui.label("Play Mode:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.play_mode,
                            PlayMode::SinglePlayer,
                            "Single Player",
                        );
                        ui.selectable_value(&mut self.play_mode, PlayMode::Networked, "Networked");
                    });

                    let enabled = self.play_mode == PlayMode::Networked;

                    if enabled {
                        ui.horizontal(|ui| {
                            ui.label("Remote Address:");
                            ui.text_edit_singleline(&mut self.remote_addr);
                        });

                        ui.add(Slider::new(&mut self.player_num, 1..=2).text("Player Number"));

                        ui.horizontal(|ui| {
                            ui.label("Local Port: ");
                            ui.text_edit_singleline(&mut self.port);
                        });
                    }
                });

                let launch_game_text = if let Some(session) = session {
                    if session.current_state() == SessionState::Synchronizing {
                        "Waiting to establish connection..."
                    } else {
                        "Connected!"
                    }
                } else {
                    "Launch Game"
                };

                ui.separator();

                ui.horizontal(|ui| {
                    let launch_game = egui::Button::new(launch_game_text);
                    if ui
                        .add_enabled(self.game_file.is_some() && session.is_none(), launch_game)
                        .clicked()
                    {
                        let path = self.game_file.as_ref().unwrap();
                        let (players, port) = match self.play_mode {
                            PlayMode::SinglePlayer => (vec![PlayerType::Local], 8000),
                            PlayMode::Networked => {
                                let remote_addr = self.remote_addr.parse::<SocketAddr>();
                                let port = self.port.parse::<u16>();

                                if remote_addr.is_err() {
                                    println!("Remote Addr is invalid");
                                    return;
                                } else if port.is_err() {
                                    println!("Port is invalid");
                                    return;
                                }

                                let player_num = self.player_num;
                                let remote_addr = remote_addr.unwrap();
                                let port = port.unwrap();

                                let players = if player_num == 1 {
                                    vec![PlayerType::Local, PlayerType::Remote(remote_addr)]
                                } else if player_num == 2 {
                                    vec![PlayerType::Remote(remote_addr), PlayerType::Local]
                                } else {
                                    println!("Player # should be 1 or 2");
                                    return;
                                };

                                (players, port)
                            }
                        };

                        let players = players.into_boxed_slice();

                        match fs::File::open(path) {
                            Err(e) => println!("fs::File::open failed: {}", e),
                            Ok(file) => {
                                let mut reader = match zstd::Decoder::new(file) {
                                    Ok(reader) => reader,
                                    Err(e) => {
                                        println!("creating decoder failed: {}", e);
                                        return;
                                    }
                                };

                                let mut buffer = Vec::new();
                                if let Err(e) = reader.read_to_end(&mut buffer) {
                                    println!("read_to_end failed: {}", e);
                                    return;
                                }

                                match bincode::deserialize_from::<_, Rom>(&*buffer) {
                                    Err(e) => println!("bincode failed: {}", e),
                                    Ok(rom) => {
                                        let num_players =
                                            if self.play_mode == PlayMode::SinglePlayer {
                                                1
                                            } else {
                                                2
                                            };

                                        let session_descriptor = SessionDescriptor {
                                            num_players,
                                            player_types: players,
                                        };

                                        pixels
                                            .resize_buffer(rom.width() as u32, rom.height() as u32);
                                        window.set_inner_size(PhysicalSize::new(
                                            rom.width().max(DEFAULT_WINDOW_RESOLUTION.width()),
                                            rom.height().max(DEFAULT_WINDOW_RESOLUTION.height()),
                                        ));

                                        let seed = u64::from_str_radix(&self.seed, 16).unwrap();

                                        let (max_prediction, new_session) = {
                                            let new_session = init_session(
                                                &rom,
                                                port,
                                                &session_descriptor.player_types,
                                            );
                                            (new_session.max_prediction(), new_session)
                                        };

                                        *session = Some(new_session);

                                        self.window_open = false;

                                        let (console, reset) = WasmConsole::new(
                                            rom,
                                            seed,
                                            session_descriptor,
                                            max_prediction,
                                        );

                                        self.wasm_console = Some(console);
                                        self.initial_state = Some(reset);
                                    }
                                }
                            }
                        };
                    };

                    let buttons_enabled = self.game_file.is_some() && session.is_some();

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Reset Game"))
                        .clicked()
                    {
                        let console = self.wasm_console.as_mut().unwrap();
                        console.load_save_state(self.initial_state.as_ref().unwrap().clone());
                    }

                    if ui
                        .add_enabled(buttons_enabled, Button::new("Quit Game"))
                        .clicked()
                    {
                        self.wasm_console = None;
                        *session = None;
                    }
                });
            });
    }
}

fn init_session(
    rom: &Rom,
    port: u16,
    players: &[PlayerType<SocketAddr>],
) -> P2PSession<WasmConsole> {
    let mut sess_builder = SessionBuilder::new()
        .with_num_players(players.len())
        .with_fps(rom.frame_rate.frames_per_second())
        .unwrap();

    for (id, address) in players.iter().enumerate() {
        sess_builder = sess_builder.add_player(*address, id).unwrap();
    }

    let socket = UdpNonBlockingSocket::bind_to_port(port).unwrap();
    sess_builder.start_p2p_session(socket).unwrap()
}
