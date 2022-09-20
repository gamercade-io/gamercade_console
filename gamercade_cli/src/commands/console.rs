use std::{
    path::{Path, PathBuf},
    process::Child,
};

use clap::{Args, Subcommand};
use gamercade_fs::EditorRom;

use crate::watch::Watchable;

use super::{read_path, try_bundle_files, ReadFileResult};

const CONSOLE_NAMES: [&str; 4] = [
    "gamercade_console",
    "console",
    "gamercade_console.exe",
    "console.exe",
];

const BUNDLE_FILENAME: &str = "bundle.gcrom";

#[derive(Subcommand, Debug, Clone)]
enum ConsoleCommand {
    /// Run a target .gcrom game.
    Rom {
        /// Path to the .gcrom file.
        rom: PathBuf,
    },

    /// Bundle and run the passed in files.
    Bundle {
        /// Path to the code provider, .wasm or .gcrom.
        #[clap(short, long, value_parser)]
        code: PathBuf,

        /// Optional path to the asset provider, .gce or .gcrom.
        #[clap(short, long, value_parser)]
        assets: Option<PathBuf>,
    },
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ConsoleArgs {
    /// Path to provide code. A .wasm or .gcrom file
    #[clap(subcommand)]
    mode: Option<ConsoleCommand>,
}

impl Watchable for ConsoleArgs {
    fn get_watch_list(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();

        match &self.mode {
            Some(ConsoleCommand::Bundle { code, assets }) => {
                out.push(code.clone());
                if let Some(assets) = assets {
                    out.push(assets.clone())
                }
            }
            Some(ConsoleCommand::Rom { rom }) => out.push(rom.clone()),
            None => (),
        }

        out
    }

    fn watchable(&self) -> bool {
        self.mode.is_some()
    }
}

pub(crate) fn run(args: &ConsoleArgs) -> Result<Option<Child>, String> {
    let console_bin = CONSOLE_NAMES
        .into_iter()
        .find(|name| Path::new(name).exists())
        .ok_or("Unable to find console binary.")?;

    let child = match &args.mode {
        Some(ConsoleCommand::Rom { rom }) => std::process::Command::new(console_bin)
            .args([
                "-g",
                rom.to_str().ok_or("Cannot convert Rom path to string.")?,
            ])
            .spawn()
            .map_err(|e| e.to_string())?,
        Some(ConsoleCommand::Bundle { code, assets }) => {
            let code = read_path(code)?;

            let assets = if let Some(assets) = assets {
                read_path(assets)?
            } else {
                ReadFileResult::EditorRom(EditorRom::default())
            };

            let rom = try_bundle_files(&code, &assets)?;
            let rom_path = PathBuf::new().with_file_name(BUNDLE_FILENAME);
            rom.try_save(&rom_path)?;

            println!("Generated new bundled rom\n");

            std::process::Command::new(console_bin)
                .args(["-g", rom_path.as_path().to_str().unwrap()])
                .spawn()
                .map_err(|e| e.to_string())?
        }
        None => std::process::Command::new(console_bin)
            .spawn()
            .map_err(|e| e.to_string())?,
    };

    Ok(Some(child))
}
