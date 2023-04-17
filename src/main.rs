mod env_file;
mod cli;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    Cli::parse().run()
}
