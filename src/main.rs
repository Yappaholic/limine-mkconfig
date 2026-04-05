mod blkid;
mod config;
mod kernel;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Set a path to save config
    #[arg(short = 'o')]
    path: Option<PathBuf>,
    /// Overwrite existing configuration file
    #[arg(short = 'O', default_value_t = false)]
    overwrite: bool,
    /// Do not output info to stdout
    #[arg(short = 'q', default_value_t = false)]
    quiet: bool,
    /// Use "uuid():/" instead of "boot():/"
    #[arg(short = 'U', default_value_t = false)]
    uuid: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    config::generate_config(args)?;
    Ok(())
}
