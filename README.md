# Gamercade Console Workspace

A Wasm powered Fantasy Console.

Learn more about [Gamercade](https://gamercade.io).

This is the main workspace crate. Consider viewing the inner crates for more information about the project. Each of them has their own readme.

- `gamercade_audio` - For all gamercade audio related things.
- `gamercade_console` - The console used to run & play games.
- `gamercade_core` - Core shared types and functionality.
- `gamercade_editor` - The editor used to bundle WASM code with assets.
- `gamercade_rs` - A safe wrapper around the raw Api.
- `gamercade_tools` - Useful assorted tools.

## Minimum Supported Rust Version

Gamercade runs on **Stable Rust 1.63 or later**.

## Building, Bundling, and Running A Game

Building, bundling, and running games requires a few different steps, which all depend on eachother.

### Building A Game (in Rust) - How to build a .wasm file

A template example project is available at: [rust_template](https://github.com/gamercade-io/rust_template)

1. If you don't already have it, install the `wasm target` by running `rustup target add wasm32-unknown-unknown`.
1. Write your game logic following the spec mentioned.
1. Build the game as a `.wasm` library. This can be done with `cargo build --target=wasm32-unknown-unknown`.
1. The file will be output in `./target/wasm32-unknown-unknown/debug/project_name.wasm`.

### Bundling A Game with the Editor - How to create a .gcrom file

1. With your game `.wasm` already built from the previous step...
1. Run the editor. This can be done from source via `cargo run --bin gamercade_editor`
1. On the File menu, click "Select game .wasm." Find and locate your previously exported `.wasm` file.
1. On the File menu, click "Export game" and export.
1. Save the `.gcrom` file in the location of your choice.

### Running a Game with the Console - How to run a .gcrom file

1. With a `.gcrom` available to play from the previous step...
1. Run the console. This can be done from source via `cargo run --bin gamercade_console`
1. The console will run with the Main Menu already opened. You can open and close it with the `Spacebar`.
1. Press the "Select Game" button to open the file dialog.
1. Select the `.gcrom` file you wish to run, and click open.
1. Launch the game by pressing the "Launch Game" button.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.