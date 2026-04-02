use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct LauncherConfig {
    pub application_id: String,
    pub window: WindowConfig,
    pub sidebar: SidebarConfig,
    pub header: HeaderBannerConfig,
    pub grid: GridSectionConfig,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub namespace: String,
    pub min_width: i32,
    pub sidebar_width: i32,
}

#[derive(Debug, Clone)]
pub struct SidebarConfig {
    pub spacing: i32,
    pub outer_margin: i32,
    pub inner_margin: i32,
    pub icon_scale: f32,
    pub middle_offset: i32,
    pub top_button: ButtonConfig,
    pub buttons: Vec<ButtonConfig>,
    pub bottom_button: ButtonConfig,
}

#[derive(Debug, Clone)]
pub struct HeaderBannerConfig {
    pub height: i32,
    pub spacing: i32,
    pub outer_margin: i32,
    pub title: String,
    pub subtitle: String,
    pub stats: Vec<HeaderStatConfig>,
    pub action_buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct HeaderStatConfig {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct GridSectionConfig {
    pub spacing: i32,
    pub columns: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    pub id: String,
    pub label: String,
    pub icon_name: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub action: MenuAction,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MenuAction {
    None,
    CloseMenu,
    OpenSection(String),
    LaunchCommand(String),
}

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(f, "failed to read config {}: {source}", path.display())
            }
            Self::Parse { path, source } => {
                write!(f, "failed to parse config {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Deserialize, Default)]
struct LauncherFileConfig {
    #[serde(default)]
    sidebar: SidebarFileConfig,
    #[serde(default)]
    grid: GridFileConfig,
}

#[derive(Debug, Deserialize, Default)]
struct SidebarFileConfig {
    icon_scale: Option<f32>,
    #[serde(default)]
    top_button: Option<ButtonFileConfig>,
    #[serde(default)]
    buttons: Vec<SidebarButtonFileEntry>,
    #[serde(default)]
    bottom_button: Option<ButtonFileConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct GridFileConfig {
    #[serde(default)]
    cards: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SidebarButtonFileEntry {
    Label(String),
    Config(ButtonFileConfig),
}

#[derive(Debug, Deserialize, Default, Clone)]
struct ButtonFileConfig {
    id: Option<String>,
    label: Option<String>,
    icon_name: Option<String>,
    icon_path: Option<PathBuf>,
}

impl LauncherConfig {
    pub fn load(config_path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut config = Self::default();

        let resolved_path = match config_path {
            Some(path) => Some(path),
            None => default_config_path().filter(|path| path.exists()),
        };

        let Some(path) = resolved_path else {
            return Ok(config);
        };

        let file_config = LauncherFileConfig::from_path(&path)?;
        config.apply_file_config(file_config, path.parent());
        Ok(config)
    }

    fn apply_file_config(&mut self, file_config: LauncherFileConfig, config_dir: Option<&Path>) {
        if let Some(icon_scale) = file_config.sidebar.icon_scale {
            self.sidebar.icon_scale = icon_scale.clamp(0.2, 1.0);
        }

        if let Some(top_button) = file_config.sidebar.top_button {
            apply_button_file_config(&mut self.sidebar.top_button, top_button, config_dir);
        }

        if !file_config.sidebar.buttons.is_empty() {
            self.sidebar.buttons = file_config
                .sidebar
                .buttons
                .into_iter()
                .map(|button| sidebar_button_config_from_file(button, config_dir))
                .collect();
        }

        if let Some(bottom_button) = file_config.sidebar.bottom_button {
            apply_button_file_config(&mut self.sidebar.bottom_button, bottom_button, config_dir);
        }

        if !file_config.grid.cards.is_empty() {
            self.grid.buttons = file_config
                .grid
                .cards
                .into_iter()
                .map(|label| grid_button_config(&label))
                .collect();
        }
    }
}

impl LauncherFileConfig {
    fn from_path(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&content).map_err(|source| ConfigError::Parse {
            path: path.to_path_buf(),
            source,
        })
    }
}

fn default_config_path() -> Option<PathBuf> {
    let config_home = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))?;

    Some(config_home.join("hyw-menu-gtk4").join("config.toml"))
}

fn sidebar_button_config(label: &str) -> ButtonConfig {
    let id = slugify(label);

    ButtonConfig {
        id: id.clone(),
        label: label.to_string(),
        icon_name: None,
        icon_path: None,
        action: MenuAction::OpenSection(id),
    }
}

fn grid_button_config(label: &str) -> ButtonConfig {
    let id = slugify(label);

    ButtonConfig {
        id: id.clone(),
        label: label.to_string(),
        icon_name: None,
        icon_path: None,
        action: MenuAction::OpenSection(id),
    }
}

fn sidebar_button_config_from_file(
    entry: SidebarButtonFileEntry,
    config_dir: Option<&Path>,
) -> ButtonConfig {
    match entry {
        SidebarButtonFileEntry::Label(label) => sidebar_button_config(&label),
        SidebarButtonFileEntry::Config(button) => {
            let label = button
                .label
                .clone()
                .or_else(|| button.id.clone())
                .unwrap_or_else(|| "Button".to_string());
            let id = button.id.unwrap_or_else(|| slugify(&label));

            ButtonConfig {
                id: id.clone(),
                label,
                icon_name: button.icon_name,
                icon_path: resolve_config_path(config_dir, button.icon_path),
                action: MenuAction::OpenSection(id),
            }
        }
    }
}

fn apply_button_file_config(
    target: &mut ButtonConfig,
    source: ButtonFileConfig,
    config_dir: Option<&Path>,
) {
    if let Some(id) = source.id {
        target.id = id;
    }

    if let Some(label) = source.label {
        target.label = label;
    }

    if let Some(icon_name) = source.icon_name {
        target.icon_name = Some(icon_name);
    }

    if let Some(icon_path) = resolve_config_path(config_dir, source.icon_path) {
        target.icon_path = Some(icon_path);
    }
}

fn resolve_config_path(config_dir: Option<&Path>, path: Option<PathBuf>) -> Option<PathBuf> {
    path.map(|path| {
        if path.is_absolute() {
            path
        } else {
            config_dir.unwrap_or_else(|| Path::new(".")).join(path)
        }
    })
}

fn slugify(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    let mut last_was_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            application_id: "com.qrzbing.hyw-menu-gtk4".to_string(),
            window: WindowConfig {
                title: "hyw-menu".to_string(),
                namespace: "hyw-menu".to_string(),
                min_width: 320,
                sidebar_width: 96,
            },
            sidebar: SidebarConfig {
                spacing: 12,
                outer_margin: 24,
                inner_margin: 16,
                icon_scale: 0.46,
                middle_offset: 40,
                top_button: ButtonConfig {
                    id: "close-menu".to_string(),
                    label: "Close".to_string(),
                    icon_name: Some("window-close-symbolic".to_string()),
                    icon_path: None,
                    action: MenuAction::CloseMenu,
                },
                buttons: vec![
                    ButtonConfig {
                        id: "all".to_string(),
                        label: "All".to_string(),
                        icon_name: Some("view-grid-symbolic".to_string()),
                        icon_path: None,
                        action: MenuAction::OpenSection("all".to_string()),
                    },
                    ButtonConfig {
                        id: "favorites".to_string(),
                        label: "Favorites".to_string(),
                        icon_name: Some("starred-symbolic".to_string()),
                        icon_path: None,
                        action: MenuAction::OpenSection("favorites".to_string()),
                    },
                    ButtonConfig {
                        id: "recent".to_string(),
                        label: "Recent".to_string(),
                        icon_name: Some("document-open-recent-symbolic".to_string()),
                        icon_path: None,
                        action: MenuAction::OpenSection("recent".to_string()),
                    },
                    ButtonConfig {
                        id: "system".to_string(),
                        label: "System".to_string(),
                        icon_name: Some("applications-system-symbolic".to_string()),
                        icon_path: None,
                        action: MenuAction::OpenSection("system".to_string()),
                    },
                ],
                bottom_button: ButtonConfig {
                    id: "power".to_string(),
                    label: "Power".to_string(),
                    icon_name: Some("system-shutdown-symbolic".to_string()),
                    icon_path: None,
                    action: MenuAction::None,
                },
            },
            header: HeaderBannerConfig {
                height: 212,
                spacing: 14,
                outer_margin: 24,
                title: "Start Menu".to_string(),
                subtitle:
                    "Game-style header placeholder. This area can later map to profile, greeting, and system status."
                        .to_string(),
                stats: vec![
                    HeaderStatConfig {
                        label: "Recent Apps".to_string(),
                        value: "12".to_string(),
                    },
                    HeaderStatConfig {
                        label: "Favorites".to_string(),
                        value: "8".to_string(),
                    },
                    HeaderStatConfig {
                        label: "System Status".to_string(),
                        value: "Normal".to_string(),
                    },
                ],
                action_buttons: vec![
                    ButtonConfig {
                        id: "edit".to_string(),
                        label: "Edit".to_string(),
                        icon_name: Some("document-edit-symbolic".to_string()),
                        icon_path: None,
                        action: MenuAction::None,
                    },
                    ButtonConfig {
                        id: "notify".to_string(),
                        label: "Alerts".to_string(),
                        icon_name: Some(
                            "preferences-system-notifications-symbolic".to_string(),
                        ),
                        icon_path: None,
                        action: MenuAction::None,
                    },
                ],
            },
            grid: GridSectionConfig {
                spacing: 12,
                columns: 4,
                tile_width: 120,
                tile_height: 96,
                buttons: vec![ButtonConfig {
                    id: "applications".to_string(),
                    label: "Applications".to_string(),
                    icon_name: Some("view-app-grid-symbolic".to_string()),
                    icon_path: None,
                    action: MenuAction::OpenSection("applications".to_string()),
                }],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn slugifies_ascii_labels() {
        assert_eq!(slugify("My Card"), "my-card");
    }

    #[test]
    fn collapses_separator_runs() {
        assert_eq!(slugify("A  /  B"), "a-b");
    }
}
