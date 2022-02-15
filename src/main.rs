mod api;
mod console;
mod core;

use crate::core::{InputState, LocalInputManager, PlayerInputEntry, Rom};
use std::{net::SocketAddr, sync::Arc};

use console::{Console, LuaConsole};
use ggrs::{Config, P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};
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
    let code_filename = "test.lua";
    let window = init_window(&event_loop, code_filename);

    let rom = Rom::default();
    let session = init_session(&rom);

    let mut pixels = init_pixels(&window, &rom);

    //TODO: Load a passed in file or from the rom
    let code = std::fs::read_to_string(code_filename).unwrap();

    // Prepare a frame buffer
    let frame_buffer = Arc::new(Mutex::new(init_frame_buffer(&rom)));

    let rom = Arc::new(rom);

    //TODO: For more players add stuff here
    let player_inputs = Arc::new(Mutex::new(
        vec![PlayerInputEntry::default()].into_boxed_slice(),
    ));

    let console = LuaConsole::new(rom, &code, frame_buffer, player_inputs.clone());
    console.call_init();

    //TODO: Incorporate Network stuff GGRS
    let mut input = WinitInputHelper::new();
    let input_manager = LocalInputManager::default();
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

            // Update internal state
            let next_input_state = input_manager.generate_input_state(&input);
            player_inputs.lock()[0].push_input_state(next_input_state);
            console.call_update();

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
fn init_session(rom: &Rom) -> P2PSession<GGRSConfig> {
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
                println!("Enter ip-address (or nothing if local):");

                let address: String = read!();

                if address.is_empty() {
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
    sess_builder.start_p2p_session(socket).unwrap()
}

#[derive(Debug)]
pub struct GGRSConfig;

impl Config for GGRSConfig {
    type Input = InputState;
    //type State = Option<()>;
    type State = Table<'static>;
    type Address = SocketAddr;
}
