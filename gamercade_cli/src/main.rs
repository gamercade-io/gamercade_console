use std::process::Child;

use clap::{Parser, Subcommand};

mod watch;

mod commands;
use commands::{bundler::BundleArgs, console::ConsoleArgs, *};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use watch::Watchable;

const WATCH_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

/// Gamercade Cli Tool.
#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Automatically re-run on file changes
    #[clap(short, long, action)]
    watch: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Bundle code and assets into a .gcrom game file.
    Bundle(BundleArgs),

    /// Run the console with optional parameters.
    Console(ConsoleArgs),
}

impl Watchable for Command {
    fn get_watch_list(&self) -> Vec<std::path::PathBuf> {
        match self {
            Command::Bundle(bundle_args) => bundle_args.get_watch_list(),
            Command::Console(console_args) => console_args.get_watch_list(),
        }
    }

    fn watchable(&self) -> bool {
        match self {
            Command::Bundle(bundle_args) => bundle_args.watchable(),
            Command::Console(console_args) => console_args.watchable(),
        }
    }
}

impl Command {
    fn run(&self) -> Result<Option<Child>, String> {
        match self {
            Command::Bundle(bundle_args) => bundler::run(bundle_args),
            Command::Console(console_args) => console::run(console_args),
        }
    }
}

pub fn main() {
    let cli = Cli::parse();

    let is_watchable = cli.command.watchable();

    if cli.watch && is_watchable {
        let mut child = match cli.command.run() {
            Ok(child) => child,
            Err(e) => {
                println!("{e}");
                return;
            }
        };

        println!("\nWatching for file changes...\n");

        let (tx, rx) = std::sync::mpsc::channel();

        let config = Config::default()
            .with_poll_interval(WATCH_POLL_INTERVAL)
            .with_compare_contents(true);

        let mut watcher = RecommendedWatcher::new(tx, config)
            .map_err(|e| e.to_string())
            .unwrap();

        cli.command.get_watch_list().into_iter().for_each(|path| {
            watcher
                .watch(path.as_path(), RecursiveMode::NonRecursive)
                .unwrap();
        });

        for res in rx {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        println!("\nFile changes detected.");

                        if let Some(child) = &mut child {
                            if let Err(e) = child.kill() {
                                println!("{e}");
                                return;
                            }
                        }

                        match cli.command.run() {
                            Ok(new_child) => child = new_child,
                            Err(e) => println!("Failed to run: {e}"),
                        }
                    }
                    _ => (),
                },
                Err(e) => println!("{e}"),
            }
        }
    } else if let Err(e) = cli.command.run() {
        println!("{e}");
    }
}
