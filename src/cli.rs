use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "hyw-menu-gtk4")]
#[command(about = "Game-style GTK4 launcher")]
pub struct Cli {
    /// Optional TOML config file path.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
