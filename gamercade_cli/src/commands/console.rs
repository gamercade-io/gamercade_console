use std::path::{Path, PathBuf};

use clap::{Args, Subcommand};

const CONSOLE_NAMES: [&str; 4] = [
    "gamercade_console",
    "console",
    "gamercade_console.exe",
    "console.exe",
];

#[derive(Args, Debug, Clone)]
pub(crate) struct ConsoleArgs {
    /// Path to provide code. A .wasm or .gcrom file
    #[clap(subcommand)]
    mode: Option<ConsoleCommand>,
}

#[derive(Subcommand, Debug, Clone)]
enum ConsoleCommand {
    /// Run a target .gcrom game.
    Rom {
        /// Path to the .gcrom file.
        rom: PathBuf,

        /// Automatically re-run on file changes
        watch: bool,
    },

    /// Bundle and run the passed in files.
    Bundle {
        /// Path to the code provider, .wasm or .gcrom.
        code: PathBuf,

        /// Optional path to the asset provider, .gce or .gcrom.
        assets: Option<PathBuf>,

        /// Automatically re-run on file changes
        watch: bool,
    },
}

pub(crate) fn run(args: &ConsoleArgs) -> Result<(), String> {
    let console_bin = CONSOLE_NAMES
        .into_iter()
        .find(|name| Path::new(name).exists())
        .ok_or_else(|| "Unable to find console binary.")?;

    match &args.mode {
        Some(ConsoleCommand::Rom { rom, watch }) => todo!(),
        Some(ConsoleCommand::Bundle {
            code,
            assets,
            watch,
        }) => todo!(),
        None => {
            std::process::Command::new(console_bin)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            Ok(())
        }
    }
}
