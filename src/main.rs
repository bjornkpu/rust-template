//! rust-template: one line on what the tool does.
//!
//! Wiring only: parse arguments, start logging, call `domain` for decisions and `io` for
//! effects. `anyhow` is used here and nowhere else.

#[cfg(feature = "tui")]
mod app;
mod domain;
mod error;
mod io;
#[cfg(feature = "tui")]
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// One line on what the tool does.
#[derive(Parser)]
// bin_name keeps help text free of ".exe" on Windows, so snapshots match on every OS.
#[command(version, bin_name = env!("CARGO_PKG_NAME"))]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print a greeting.
    Greet {
        /// Who to greet.
        #[arg(default_value = "world")]
        name: String,
    },
    /// Open the terminal UI.
    #[cfg(feature = "tui")]
    Tui,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let home = domain::paths::home(
        std::env::var_os("RUST_TEMPLATE_HOME").map(Into::into),
        std::env::home_dir(),
    )?;
    let _log_guard = io::log::init(&home.join("logs"))?;
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "starting");
    match cli.command {
        Command::Greet { name } => {
            tracing::debug!(%name, "greet");
            println!("{}", domain::greet::greeting(&name));
        }
        #[cfg(feature = "tui")]
        Command::Tui => io::terminal::run()?,
    }
    Ok(())
}
