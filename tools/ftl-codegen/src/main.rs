mod cli;
mod parser;
mod diff;
mod check;
mod codegen;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { dir } => {
            check::run(&dir)?;
        }
        Commands::GenCode { codes_path, locales_dir,out} => {
            codegen::run(&codes_path, &locales_dir, &out)?; 
        }
    }

    Ok(())
}
