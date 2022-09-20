# Gamercade Console Workspace

The ultimate WASM powered Fantasy Console.

Learn more about [Gamercade](https://gamercade.io).

## Features

- **Simple and Powerful** - 2d Rendering, Input, and Audio all included.
- **Language Agnostic** - Write in your favorite language and compile to WebAssembly.
- **Tool Agnostic** - Build assets using your favorite tools. Bundle them with the Editor.
- **Seamless Multiplayer** - Built-in p2p rollback lets you get multiplayer "for free."
- **Collaboration Friendly** - Save work-in-progress as JSON, for easy collaboration.
- **Free and Open Source** - Free to build, use, and play, forever.

## Motivation

Gamercade aims to solve the "I'm a small team/solo developer and I want to make a multiplayer game" problem. In order to have a successful game, these small projects need to have multiple successes:

- Building a project which correctly handles the complexities of online multiplayer.
- Marketing to a big enough playerbase to shorten wait times.
- Setting up server hosting, and the associated costs to keep them running well after release.
- Making a game which is actually fun to play.

With Gamercade, the first 3 requirements are removed. This creates a more modern development environment which lets game developers do what they do best: Make exciting games!

## Project Goals

Every feature and function of Gamercade and its related tools are built with the goals of achieving the following:

- **Effortless Multiplayer** - The main priority is developing a top-notch multiplayer experience for both players and developers. This means providing an easy-to-use networking solution for developers, and also one which is robust and high performant.
- **Empower Creatives** - Gamercade is a platform for all kinds of people, of different backgrounds and experience levels. Programmers, designers, artists, and are all welcome. Gamercade should empower creators and allow them to always do their best work.
- **"Neo Retro" Game Development** - Project scoping is important. Retro consoles are cool, but also constrained and complex. Gamercade provides the balance between retro and modern development. Games are limited by content, but creativity is unlimited.

## Related Projects:

 - [**gamercade_site**](https://github.com/gamercade-io/gamercade_site) - Project landing page and [documentation](https://gamercade.io/docs/intro).
 - **Bindings and Example Projects**
    - [**rust_template**](https://github.com/gamercade-io/rust_template) - Rust example project, making use of the `gamercade_rs` helper crate.
    - [**nelua-gamercade**](https://codeberg.org/Andre-LA/nelua-gamercade) - Bindings and example game in the [Nelua](https://nelua.io/) programming language. By [Andre-LA](https://codeberg.org/Andre-LA).
    - [**zig-template**](https://github.com/gamercade-io/zig-template) - Bindings and starter project for the [Zig](https://ziglang.org/) programming language.
    - [**gamercade-as**](https://github.com/NEARworld/gamercade-as) - Bindings and starter project for the [AssemblyScript](https://www.assemblyscript.org/) programming language. By [NEARworld](https://github.com/NEARworld).

## Community

Currently, the community is mostly active on [Discord](https://discord.gg/Qafv2Fpt5j).

## FAQ

We recommend reading the [FAQ](https://gamercade.io/faq) on our home page.

# For Developers

Follow these steps to get your first game project up and running. See the included resources for more information. Also consider making yourself comfortable with the [Api Reference](https://gamercade.io/docs/api-reference).

## Building, Bundling, and Running A Game

Building, bundling, and running games requires a few different steps, which all depend on eachother. Below are steps on how to accomplish this using the editor through the GUI. Alternatively, a more efficient way can be done via the `gccl` tool. See the `gamercade_cli` folder for more information about how to invoke and use `gccl`.

### Building A Game (in Rust) - How to build a .wasm file

A template example project is available at: [rust_template](https://github.com/gamercade-io/rust_template)

1. If you don't already have it, install the `wasm target` by running `rustup target add wasm32-unknown-unknown`.
1. Write your game logic following the spec mentioned.
1. Build the game as a `.wasm` library. This can be done with `cargo build --target=wasm32-unknown-unknown`.
1. The file will be output in `./target/wasm32-unknown-unknown/debug/project_name.wasm`.

### Bundling A Game with the Editor - How to create a .gcrom file

1. With your game `.wasm` already built from the previous steps...
1. Run the editor. This can be done from source via `cargo run --bin editor`
1. On the File menu, click "Select game .wasm." Find and locate your previously exported `.wasm` file.
1. On the File menu, click "Export game" and export.
1. Save the `.gcrom` file in the location of your choice.

### Running a Game with the Console - How to run a .gcrom file

1. With a `.gcrom` available to play from the previous steps...
1. Run the console. This can be done from source via `cargo run --bin console`
1. The console will run with the Main Menu already opened. You can open and close it with the `Spacebar`.
1. Press the "Select Game" button to open the file dialog.
1. Select the `.gcrom` file you wish to run, and click open.
1. Launch the game by pressing the "Launch Game" button.

# For Contributors

This is the main workspace crate. Consider viewing the inner crates for more information about the project. Each of them has their own README.

- `gamercade_audio` - For all gamercade audio related things.
- `gamercade_cli` - The `gccl` tools for development.
- `gamercade_console` - The console used to run & play games.
- `gamercade_core` - Core shared types and functionality.
- `gamercade_editor` - The editor used to bundle WASM code with assets.
- `gamercade_fs` - File System management, loading, saving etc.
- `gamercade_rs` - A safe wrapper around the raw Api.
- `gamercade_sound_engine` - Closely related to gamercade_audio, responsible for actual sound output.
- `gamercade_tools` - Useful assorted tools.

## Minimum Supported Rust Version

Gamercade runs on **Stable Rust 1.63 or later**.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
