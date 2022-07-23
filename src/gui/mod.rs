use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};

use egui::{Context, Slider};
use gamercade_core::Rom;
use ggrs::{P2PSession, PlayerType, SessionBuilder, SessionState, UdpNonBlockingSocket};
use pixels::Pixels;
use rfd::FileDialog;

use crate::console::WasmConsole;

pub mod framework;

pub struct Gui {
    pub window_open: bool,
    pub game_file: Option<PathBuf>,
    pub play_mode: PlayMode,
    pub remote_addr: String,
    pub player_num: usize,
    pub port: String,

    pub wasm_console: Option<WasmConsole>,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            window_open: true,
            game_file: None,
            play_mode: PlayMode::SinglePlayer,
            remote_addr: String::new(),
            player_num: 1,
            port: String::new(),
            wasm_console: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayMode {
    SinglePlayer,
    Networked,
}

impl Gui {
    fn ui(
        &mut self,
        pixels: &mut Pixels,
        session: &mut Option<P2PSession<WasmConsole>>,
        ctx: &Context,
    ) {
        egui::Window::new("Main Menu")
            .open(&mut self.window_open)
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

                    match fs::read(path) {
                        Err(e) => println!("{}", e),
                        Ok(bin) => match bincode::deserialize_from::<_, Rom>(&*bin) {
                            Err(e) => println!("{}", e),
                            Ok(rom) => {
                                let num_players = if self.play_mode == PlayMode::SinglePlayer {
                                    1
                                } else {
                                    2
                                };

                                pixels.resize_buffer(rom.width() as u32, rom.height() as u32);
                                *session = Some(init_session(&rom, port, &players));
                                self.wasm_console =
                                    Some(WasmConsole::new(Arc::new(rom), num_players));
                            }
                        },
                    };
                }
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
