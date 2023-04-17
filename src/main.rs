mod env_file;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Environment Variable Control
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Update(update::Args),
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Update(args) => update::run(args),
    }
}
