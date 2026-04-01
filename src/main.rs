mod app;
mod config;
mod hyprland;

use app::LauncherApp;
use config::LauncherConfig;

fn main() {
    LauncherApp::new(LauncherConfig::default()).run();
}
