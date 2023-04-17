mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Environment Variable Control
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Update(update::Cmd),
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Update(cmd) => cmd.run(),
        }
    }
}
