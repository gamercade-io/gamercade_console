use clap::{Parser, Subcommand};

mod commands;
use commands::{bundler::BundleArgs, *};

const WATCH_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

/// Gamercade Cli Tool.
#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Bundle code and assets into a .gcrom game file.
    Bundle(BundleArgs),
}

pub fn main() {
    let cli = Cli::parse();

    if let Err(e) = match &cli.command {
        Command::Bundle(bundle_args) => bundler::run(bundle_args),
    } {
        println!("{}", e);
    }
}
