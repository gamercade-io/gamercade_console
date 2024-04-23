use std::{fs, path::PathBuf, process::Child};

use clap::Args;
use gamercade_fs::{DataPack, EditorRom};

use crate::{commands::try_bundle_files, watch::Watchable};

use super::{read_path, ReadFileResult};

#[derive(Args, Debug, Clone)]
pub(crate) struct BundleArgs {
    /// Path to provide code. A .wasm or .gcrom file
    #[clap(short, long, value_parser)]
    code: PathBuf,

    /// Path to provide game assets. A .gce or .gcrom file
    #[clap(short, long, value_parser)]
    assets: Option<PathBuf>,

    /// Path to provide a game data pack. Any file type.
    #[clap(short, long, value_parser)]
    data_pack: Option<PathBuf>,

    /// Path of the output file.
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

impl Watchable for BundleArgs {
    fn get_watch_list(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();

        out.push(self.code.clone());

        if let Some(path) = &self.assets {
            out.push(path.clone())
        };

        out
    }

    fn watchable(&self) -> bool {
        true
    }
}

pub(crate) fn run(args: &BundleArgs) -> Result<Option<Child>, String> {
    let code = read_path(&args.code)?;

    let mut assets = if let Some(assets) = &args.assets {
        read_path(assets)?
    } else {
        println!("No assets provided, using default data.");
        ReadFileResult::EditorRom(EditorRom::default())
    };

    if let Some(data_pack) = &args.data_pack {
        let data = fs::read(data_pack).unwrap();
        assets.set_data_pack(DataPack { data });
    }

    let path = match args
        .output
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("gcrom") => args.output.clone(),
        _ => args.output.clone().with_extension("gcrom"),
    };

    let bundled_rom = try_bundle_files(&code, &assets)?;
    bundled_rom.try_save(&path)?;

    println!("Bundled rom output to: {}", path.to_string_lossy());
    Ok(None)
}
