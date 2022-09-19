use std::path::PathBuf;

use clap::Args;
use gamercade_fs::{bundle, EditorRom};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use super::{read_path, ReadFileResult};

#[derive(Args, Debug, Clone)]
pub(crate) struct BundleArgs {
    /// Path to provide code. A .wasm or .gcrom file
    #[clap(short, long, value_parser)]
    code: PathBuf,

    /// Path to provide game assets. A .gce or .gcrom file
    #[clap(short, long, value_parser)]
    assets: Option<PathBuf>,

    /// Path of the output file.
    #[clap(short, long, value_parser)]
    output: PathBuf,

    #[clap(short, long, action)]
    watch: bool,
}

pub(crate) fn run(args: &BundleArgs) -> Result<(), String> {
    if args.watch {
        println!("\nWatching for file changes...\n");

        let (tx, rx) = std::sync::mpsc::channel();

        let config = Config::default()
            .with_poll_interval(std::time::Duration::from_secs(1))
            .with_compare_contents(true);

        let mut watcher = RecommendedWatcher::new(tx, config).map_err(|e| e.to_string())?;

        watcher
            .watch(args.code.as_path(), RecursiveMode::NonRecursive)
            .unwrap();

        if let Some(path) = &args.assets {
            watcher
                .watch(path.as_path(), RecursiveMode::NonRecursive)
                .unwrap();
        }

        for res in rx {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        println!("\nFile changes detected.");
                        match try_bundle(args) {
                            Ok(_) => (),
                            Err(e) => println!("Failed to bundle: {}", e),
                        }
                    }
                    _ => (),
                },
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    } else {
        try_bundle(args)
    }
}

fn try_bundle(args: &BundleArgs) -> Result<(), String> {
    let code = read_path(&args.code)?;

    if let ReadFileResult::EditorRom(..) = code {
        return Err("Code provider must be a .wasm or .gcrom".to_string());
    }

    let assets = if let Some(assets) = &args.assets {
        read_path(assets)?
    } else {
        println!("No assets provided, using default data.");
        ReadFileResult::EditorRom(EditorRom::default())
    };

    if let ReadFileResult::Code(..) = assets {
        return Err("Asset provider must be a .gce or .gcrom".to_string());
    }

    let bundled_rom = match (&code, &assets) {
        (ReadFileResult::Rom(rom1), ReadFileResult::Rom(rom2)) => bundle(rom1, rom2),
        (ReadFileResult::Rom(rom), ReadFileResult::EditorRom(editor_rom)) => {
            bundle(rom, editor_rom)
        }
        (ReadFileResult::Code(code), ReadFileResult::Rom(rom)) => bundle(code, rom),
        (ReadFileResult::Code(code), ReadFileResult::EditorRom(editor_rom)) => {
            bundle(code, editor_rom)
        }
        _ => unreachable!(),
    };

    let path = match args
        .output
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("gcrom") => args.output.clone(),
        _ => args.output.clone().with_extension("gcrom"),
    };

    bundled_rom.try_save(&path)?;
    println!("Bundled rom output to: {}", path.to_string_lossy());
    Ok(())
}
