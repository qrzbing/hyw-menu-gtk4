use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct LauncherConfig {
    pub application_id: String,
    pub quick_access_path: PathBuf,
    pub theme: ThemeConfig,
    pub window: WindowConfig,
    pub sidebar: SidebarConfig,
    pub header: HeaderBannerConfig,
    pub grid: GridSectionConfig,
}

#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub font_family: Option<String>,
    pub font_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub namespace: String,
    pub min_width: i32,
    pub width_ratio: f32,
    pub height_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct SidebarConfig {
    pub width: i32,
    pub scale: f32,
    pub button_scale: f32,
    pub button_bg_opacity: f32,
    pub spacing: i32,
    pub outer_margin: i32,
    pub inner_margin: i32,
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
    pub tile_size: i32,
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
    theme: ThemeFileConfig,
    #[serde(default)]
    window: WindowFileConfig,
    #[serde(default)]
    sidebar: SidebarFileConfig,
    #[serde(default)]
    grid: GridFileConfig,
}

#[derive(Debug, Deserialize, Default)]
struct ThemeFileConfig {
    font_family: Option<String>,
    font_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Default)]
struct WindowFileConfig {
    min_width: Option<i32>,
    width_ratio: Option<f32>,
    height_ratio: Option<f32>,
}

#[derive(Debug, Deserialize, Default)]
struct SidebarFileConfig {
    width: Option<i32>,
    scale: Option<f32>,
    button_scale: Option<f32>,
    button_bg_opacity: Option<f32>,
    #[serde(default)]
    top_button: Option<ButtonFileConfig>,
    #[serde(default)]
    buttons: Vec<SidebarButtonFileEntry>,
    #[serde(default)]
    bottom_button: Option<ButtonFileConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct GridFileConfig {
    tile_size: Option<i32>,
    tile_width: Option<i32>,
    tile_height: Option<i32>,
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
    #[serde(alias = "name")]
    label: Option<String>,
    icon: Option<String>,
    #[serde(alias = "icon_name")]
    icon_name: Option<String>,
    #[serde(alias = "icon_path")]
    icon_path: Option<PathBuf>,
}

impl LauncherConfig {
    pub fn load(config_path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        let config_dir = config_root_dir(config_path.as_deref());
        config.quick_access_path = config_dir.join("quick-access.toml");

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
        if let Some(font_family) = file_config.theme.font_family {
            let trimmed = font_family.trim();
            self.theme.font_family = (!trimmed.is_empty()).then(|| trimmed.to_string());
        }

        if let Some(font_path) = resolve_config_path(config_dir, file_config.theme.font_path) {
            self.theme.font_path = Some(font_path);
        }

        if let Some(min_width) = file_config.window.min_width {
            self.window.min_width = min_width.max(320);
        }

        if let Some(width_ratio) = file_config.window.width_ratio {
            self.window.width_ratio = width_ratio.clamp(0.25, 0.5);
        }

        if let Some(height_ratio) = file_config.window.height_ratio {
            self.window.height_ratio = height_ratio.clamp(0.5, 0.95);
        }

        if let Some(width) = file_config.sidebar.width {
            self.sidebar.width = width.max(56);
        }

        if let Some(scale) = file_config.sidebar.scale {
            self.sidebar.scale = scale.clamp(0.5, 2.0);
        }

        if let Some(button_scale) = file_config.sidebar.button_scale {
            self.sidebar.button_scale = button_scale.clamp(0.4, 1.0);
        }

        if let Some(button_bg_opacity) = file_config.sidebar.button_bg_opacity {
            self.sidebar.button_bg_opacity = button_bg_opacity.clamp(0.0, 1.0);
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

        let legacy_tile_size = file_config.grid.tile_width.or(file_config.grid.tile_height);
        if let Some(tile_size) = file_config.grid.tile_size.or(legacy_tile_size) {
            self.grid.tile_size = tile_size.clamp(72, 192);
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
    Some(default_config_dir().join("config.toml"))
}

fn default_config_dir() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("hyw-menu-gtk4")
}

fn config_root_dir(config_path: Option<&Path>) -> PathBuf {
    config_path
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(default_config_dir)
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
            let default_text = button_display_text(
                button.label.as_ref(),
                button.id.as_ref(),
                button.icon_path.as_ref(),
            );
            let label = default_text.clone().unwrap_or_else(|| "Button".to_string());
            let (icon_name, icon_path) = resolve_button_icon_fields(&button, config_dir);
            let id = button.id.unwrap_or_else(|| slugify(&label));

            ButtonConfig {
                id: id.clone(),
                label,
                icon_name,
                icon_path,
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
    let resolved_label = button_display_text(
        source.label.as_ref(),
        source.id.as_ref(),
        source.icon_path.as_ref(),
    );
    let (icon_name, icon_path) = resolve_button_icon_fields(&source, config_dir);

    if let Some(id) = source.id {
        target.id = id;
    }

    if let Some(label) = resolved_label {
        target.label = label;
    }

    if let Some(icon_name) = icon_name {
        target.icon_name = Some(icon_name);
    }

    if let Some(icon_path) = icon_path {
        target.icon_path = Some(icon_path);
    }
}

fn button_display_text(
    label: Option<&String>,
    id: Option<&String>,
    icon_path: Option<&PathBuf>,
) -> Option<String> {
    label
        .cloned()
        .or_else(|| id.cloned())
        .or_else(|| icon_path.and_then(path_file_stem))
}

fn path_file_stem(path: &PathBuf) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_string())
}

fn resolve_button_icon_fields(
    source: &ButtonFileConfig,
    config_dir: Option<&Path>,
) -> (Option<String>, Option<PathBuf>) {
    if let Some(icon) = &source.icon {
        return if looks_like_path(icon) {
            (
                None,
                resolve_config_path(config_dir, Some(PathBuf::from(icon))),
            )
        } else {
            (Some(icon.clone()), None)
        };
    }

    (
        source.icon_name.clone(),
        resolve_config_path(config_dir, source.icon_path.clone()),
    )
}

fn looks_like_path(icon: &str) -> bool {
    icon.contains('/')
        || icon.contains('\\')
        || icon.starts_with('.')
        || icon.ends_with(".png")
        || icon.ends_with(".svg")
        || icon.ends_with(".jpg")
        || icon.ends_with(".jpeg")
        || icon.ends_with(".webp")
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
            quick_access_path: default_config_dir().join("quick-access.toml"),
            theme: ThemeConfig {
                font_family: None,
                font_path: None,
            },
            window: WindowConfig {
                title: "hyw-menu".to_string(),
                namespace: "hyw-menu".to_string(),
                min_width: 560,
                width_ratio: 0.34,
                height_ratio: 0.82,
            },
            sidebar: SidebarConfig {
                width: 96,
                scale: 1.0,
                button_scale: 0.82,
                button_bg_opacity: 1.0,
                spacing: 12,
                outer_margin: 24,
                inner_margin: 16,
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
                tile_size: 96,
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
