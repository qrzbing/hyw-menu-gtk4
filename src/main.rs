mod app;
mod cli;
mod config;
mod hyprland;

use app::LauncherApp;
use clap::Parser;
use cli::Cli;
use config::LauncherConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = LauncherConfig::load(cli.config)?;

    LauncherApp::new(config).run();
    Ok(())
}
