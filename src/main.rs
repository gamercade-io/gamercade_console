mod api;
mod console;
mod core;

use crate::core::{FrameRate, InputState, LocalInputManager, PlayerInputEntry, Rom};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use console::{Console, LuaConsole};
use ggrs::{
    Config, GGRSError, P2PSession, PlayerType, SessionBuilder, SessionState, UdpNonBlockingSocket,
};
use parking_lot::Mutex;
use pixels::{Error, Pixels, SurfaceTexture};
use rlua::Table;
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let code_filename = "test2.lua";
    let window = init_window(&event_loop, code_filename);

    let mut rom = Rom::default();
    rom.frame_rate = FrameRate::Fast;
    let (num_players, mut session) = init_session(&rom);

    let mut pixels = init_pixels(&window, &rom);

    //TODO: Load a passed in file or from the rom
    let code = std::fs::read_to_string(code_filename).unwrap();

    // Prepare a frame buffer
    let frame_buffer = Arc::new(Mutex::new(init_frame_buffer(&rom)));

    let rom = Arc::new(rom);

    //TODO: For more players add stuff here
    let player_inputs = Arc::new(Mutex::new(
        (0..num_players)
            .map(|_| PlayerInputEntry::default())
            .collect(),
    ));

    let mut console = LuaConsole::new(rom.clone(), &code, frame_buffer, player_inputs.clone());
    console.call_init();

    //TODO: Incorporate Network stuff GGRS
    let mut input = WinitInputHelper::new();
    let input_manager = LocalInputManager::default();
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Handle GGRS packets
            session.poll_remote_clients();

            if session.current_state() == SessionState::Running {
                // this is to keep ticks between clients synchronized.
                // if a client is ahead, it will run frames slightly slower to allow catching up
                let mut fps_delta = 1. / rom.frame_rate.frames_per_second() as f64;
                if session.frames_ahead() > 0 {
                    fps_delta *= 1.1;
                }

                // get delta time from last iteration and accumulate it
                let delta = Instant::now().duration_since(last_update);
                accumulator = accumulator.saturating_add(delta);
                last_update = Instant::now();

                while accumulator.as_secs_f64() > fps_delta {
                    accumulator = accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

                    // Generate all local inputs
                    // TODO: Refactor this to handle multiple local players
                    for handle in session.local_player_handles() {
                        session
                            .add_local_input(handle, input_manager.generate_input_state(&input))
                            .unwrap();
                    }

                    //TODO: What to do with this??
                    //player_inputs.lock()[0].push_input_state(next_input_state);

                    // Update internal state
                    match session.advance_frame() {
                        Ok(requests) => {
                            console.handle_requests(requests);
                        }
                        Err(GGRSError::PredictionThreshold) => (),
                        Err(e) => panic!("{}", e),
                    }
                }

                // Render the game
                console.call_draw();
                console.blit(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| println!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                window.request_redraw();
            }
        }
    });
}

fn init_window(event_loop: &EventLoop<()>, code_filename: &str) -> Window {
    let size = LogicalSize::new(320_f64, 180_f64);
    WindowBuilder::new()
        .with_title(format!(
            "Gamercade Console - {} - {}x{}",
            code_filename, size.width, size.height
        ))
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

fn init_pixels(window: &Window, rom: &Rom) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let (width, height) = (rom.resolution.width(), rom.resolution.height());

    Pixels::new(width, height, surface_texture).unwrap()
}

fn init_frame_buffer(rom: &Rom) -> Box<[u8]> {
    (0..rom.resolution.total_pixels() * 4)
        .map(|_| 0)
        .collect::<Vec<u8>>()
        .into_boxed_slice()
}

// TODO: Finish this GGRS Related things:
fn init_session(rom: &Rom) -> (usize, P2PSession<GGRSConfig>) {
    use text_io::read;

    println!("Enter port number:");
    let port: u16 = read!();

    println!("Enter number of players (1-4): ");
    let num_players: usize = read!();

    assert!(num_players > 0);
    assert!(num_players <= 4);

    let player_ips = if num_players == 1 {
        vec![PlayerType::<SocketAddr>::Local]
    } else {
        (0..num_players)
            .map(|_| {
                println!("Enter ip-address (or 'local' for local):");

                let address: String = read!();

                if address == "local" {
                    PlayerType::Local
                } else {
                    PlayerType::Remote(address.parse().unwrap())
                }
            })
            .collect()
    };

    let mut sess_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_fps(rom.frame_rate.frames_per_second())
        .unwrap();

    for (id, address) in player_ips.into_iter().enumerate() {
        sess_builder = sess_builder.add_player(address, id).unwrap();
    }

    let socket = UdpNonBlockingSocket::bind_to_port(port).unwrap();
    (num_players, sess_builder.start_p2p_session(socket).unwrap())
}

#[derive(Debug)]
pub struct GGRSConfig;

impl Config for GGRSConfig {
    type Input = InputState;
    type State = Box<[PlayerInputEntry]>;
    type Address = SocketAddr;
}
