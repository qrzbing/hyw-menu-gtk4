use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct LauncherConfig {
    application_id: String,
    quick_access_path: PathBuf,
    theme: ThemeConfig,
    window: WindowConfig,
    sidebar: SidebarConfig,
    header: HeaderBannerConfig,
    grid: GridSectionConfig,
}

#[derive(Debug, Clone)]
pub struct ThemeConfig {
    font_family: Option<String>,
    font_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    namespace: String,
    min_width: i32,
    width_ratio: f32,
    height_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct SidebarConfig {
    width: i32,
    scale: f32,
    button_scale: f32,
    button_bg_opacity: f32,
    spacing: i32,
    outer_margin: i32,
    inner_margin: i32,
    middle_offset: i32,
    top_button: ButtonConfig,
    buttons: Vec<ButtonConfig>,
    bottom_button: ButtonConfig,
}

#[derive(Debug, Clone)]
pub struct HeaderBannerConfig {
    height: i32,
    spacing: i32,
    outer_margin: i32,
    title: String,
    subtitle: String,
    stats: Vec<HeaderStatConfig>,
    action_buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct HeaderStatConfig {
    label: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct GridSectionConfig {
    spacing: i32,
    columns: i32,
    tile_size: i32,
    buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    id: String,
    label: String,
    icon_name: Option<String>,
    icon_path: Option<PathBuf>,
    action: MenuAction,
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
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    pub fn quick_access_path(&self) -> &Path {
        &self.quick_access_path
    }

    pub fn set_quick_access_path(&mut self, quick_access_path: PathBuf) {
        self.quick_access_path = quick_access_path;
    }

    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }

    pub fn window(&self) -> &WindowConfig {
        &self.window
    }

    pub fn sidebar(&self) -> &SidebarConfig {
        &self.sidebar
    }

    pub fn header(&self) -> &HeaderBannerConfig {
        &self.header
    }

    pub fn grid(&self) -> &GridSectionConfig {
        &self.grid
    }

    pub fn load(config_path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        let config_dir = config_root_dir(config_path.as_deref());
        config.set_quick_access_path(config_dir.join("quick-access.toml"));

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
            self.theme
                .set_font_family((!trimmed.is_empty()).then(|| trimmed.to_owned()));
        }

        if let Some(font_path) = resolve_config_path(config_dir, file_config.theme.font_path) {
            self.theme.set_font_path(Some(font_path));
        }

        if let Some(min_width) = file_config.window.min_width {
            self.window.set_min_width(min_width.max(320));
        }

        if let Some(width_ratio) = file_config.window.width_ratio {
            self.window.set_width_ratio(width_ratio.clamp(0.25, 0.5));
        }

        if let Some(height_ratio) = file_config.window.height_ratio {
            self.window.set_height_ratio(height_ratio.clamp(0.5, 0.95));
        }

        if let Some(width) = file_config.sidebar.width {
            self.sidebar.set_width(width.max(56));
        }

        if let Some(scale) = file_config.sidebar.scale {
            self.sidebar.set_scale(scale.clamp(0.5, 2.0));
        }

        if let Some(button_scale) = file_config.sidebar.button_scale {
            self.sidebar.set_button_scale(button_scale.clamp(0.4, 1.0));
        }

        if let Some(button_bg_opacity) = file_config.sidebar.button_bg_opacity {
            self.sidebar
                .set_button_bg_opacity(button_bg_opacity.clamp(0.0, 1.0));
        }

        if let Some(top_button) = file_config.sidebar.top_button {
            apply_button_file_config(self.sidebar.top_button_mut(), top_button, config_dir);
        }

        if !file_config.sidebar.buttons.is_empty() {
            let buttons = file_config
                .sidebar
                .buttons
                .into_iter()
                .map(|button| sidebar_button_config_from_file(button, config_dir))
                .collect();
            self.sidebar.set_buttons(buttons);
        }

        if let Some(bottom_button) = file_config.sidebar.bottom_button {
            apply_button_file_config(self.sidebar.bottom_button_mut(), bottom_button, config_dir);
        }

        if !file_config.grid.cards.is_empty() {
            let buttons = file_config
                .grid
                .cards
                .into_iter()
                .map(|label| grid_button_config(&label))
                .collect();
            self.grid.set_buttons(buttons);
        }

        let legacy_tile_size = file_config.grid.tile_width.or(file_config.grid.tile_height);
        if let Some(tile_size) = file_config.grid.tile_size.or(legacy_tile_size) {
            self.grid.set_tile_size(tile_size.clamp(72, 192));
        }
    }
}

impl ThemeConfig {
    pub fn font_family(&self) -> Option<&str> {
        self.font_family.as_deref()
    }

    pub fn font_path(&self) -> Option<&Path> {
        self.font_path.as_deref()
    }

    pub fn set_font_family(&mut self, font_family: Option<String>) {
        self.font_family = font_family;
    }

    pub fn set_font_path(&mut self, font_path: Option<PathBuf>) {
        self.font_path = font_path;
    }
}

impl WindowConfig {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn min_width(&self) -> i32 {
        self.min_width
    }

    pub fn width_ratio(&self) -> f32 {
        self.width_ratio
    }

    pub fn height_ratio(&self) -> f32 {
        self.height_ratio
    }

    pub fn set_min_width(&mut self, min_width: i32) {
        self.min_width = min_width;
    }

    pub fn set_width_ratio(&mut self, width_ratio: f32) {
        self.width_ratio = width_ratio;
    }

    pub fn set_height_ratio(&mut self, height_ratio: f32) {
        self.height_ratio = height_ratio;
    }
}

impl SidebarConfig {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn button_scale(&self) -> f32 {
        self.button_scale
    }

    pub fn button_bg_opacity(&self) -> f32 {
        self.button_bg_opacity
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
    }

    pub fn inner_margin(&self) -> i32 {
        self.inner_margin
    }

    pub fn middle_offset(&self) -> i32 {
        self.middle_offset
    }

    pub fn top_button(&self) -> &ButtonConfig {
        &self.top_button
    }

    pub fn buttons(&self) -> &[ButtonConfig] {
        &self.buttons
    }

    pub fn bottom_button(&self) -> &ButtonConfig {
        &self.bottom_button
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_button_scale(&mut self, button_scale: f32) {
        self.button_scale = button_scale;
    }

    pub fn set_button_bg_opacity(&mut self, button_bg_opacity: f32) {
        self.button_bg_opacity = button_bg_opacity;
    }

    pub fn set_buttons(&mut self, buttons: Vec<ButtonConfig>) {
        self.buttons = buttons;
    }

    pub fn top_button_mut(&mut self) -> &mut ButtonConfig {
        &mut self.top_button
    }

    pub fn bottom_button_mut(&mut self) -> &mut ButtonConfig {
        &mut self.bottom_button
    }
}

impl HeaderBannerConfig {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }

    pub fn stats(&self) -> &[HeaderStatConfig] {
        &self.stats
    }

    pub fn action_buttons(&self) -> &[ButtonConfig] {
        &self.action_buttons
    }
}

impl HeaderStatConfig {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GridSectionConfig {
    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn columns(&self) -> i32 {
        self.columns
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }

    pub fn set_tile_size(&mut self, tile_size: i32) {
        self.tile_size = tile_size;
    }

    pub fn set_buttons(&mut self, buttons: Vec<ButtonConfig>) {
        self.buttons = buttons;
    }
}

impl ButtonConfig {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn icon_name(&self) -> Option<&str> {
        self.icon_name.as_deref()
    }

    pub fn icon_path(&self) -> Option<&Path> {
        self.icon_path.as_deref()
    }

    pub fn action(&self) -> &MenuAction {
        &self.action
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_icon_name(&mut self, icon_name: Option<String>) {
        self.icon_name = icon_name;
    }

    pub fn set_icon_path(&mut self, icon_path: Option<PathBuf>) {
        self.icon_path = icon_path;
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
        label: label.to_owned(),
        icon_name: None,
        icon_path: None,
        action: MenuAction::OpenSection(id),
    }
}

fn grid_button_config(label: &str) -> ButtonConfig {
    let id = slugify(label);

    ButtonConfig {
        id: id.clone(),
        label: label.to_owned(),
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
                button.label.as_deref(),
                button.id.as_deref(),
                button.icon_path.as_deref(),
            );
            let label = default_text.clone().unwrap_or_else(|| "Button".to_owned());
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
        source.label.as_deref(),
        source.id.as_deref(),
        source.icon_path.as_deref(),
    );
    let (icon_name, icon_path) = resolve_button_icon_fields(&source, config_dir);

    if let Some(id) = source.id {
        target.set_id(id);
    }

    if let Some(label) = resolved_label {
        target.set_label(label);
    }

    if let Some(icon_name) = icon_name {
        target.set_icon_name(Some(icon_name));
    }

    if let Some(icon_path) = icon_path {
        target.set_icon_path(Some(icon_path));
    }
}

fn button_display_text(
    label: Option<&str>,
    id: Option<&str>,
    icon_path: Option<&Path>,
) -> Option<String> {
    label
        .map(str::to_owned)
        .or_else(|| id.map(str::to_owned))
        .or_else(|| icon_path.and_then(path_file_stem))
}

fn path_file_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_owned)
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
            (Some(icon.to_owned()), None)
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

    slug.trim_matches('-').to_owned()
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            application_id: "com.qrzbing.hyw-menu-gtk4".to_owned(),
            quick_access_path: default_config_dir().join("quick-access.toml"),
            theme: ThemeConfig {
                font_family: None,
                font_path: None,
            },
            window: WindowConfig {
                title: "hyw-menu".to_owned(),
                namespace: "hyw-menu".to_owned(),
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
                    id: "close-menu".to_owned(),
                    label: "Close".to_owned(),
                    icon_name: Some("window-close-symbolic".to_owned()),
                    icon_path: None,
                    action: MenuAction::CloseMenu,
                },
                buttons: vec![
                    ButtonConfig {
                        id: "all".to_owned(),
                        label: "All".to_owned(),
                        icon_name: Some("view-grid-symbolic".to_owned()),
                        icon_path: None,
                        action: MenuAction::OpenSection("all".to_owned()),
                    },
                    ButtonConfig {
                        id: "favorites".to_owned(),
                        label: "Favorites".to_owned(),
                        icon_name: Some("starred-symbolic".to_owned()),
                        icon_path: None,
                        action: MenuAction::OpenSection("favorites".to_owned()),
                    },
                    ButtonConfig {
                        id: "recent".to_owned(),
                        label: "Recent".to_owned(),
                        icon_name: Some("document-open-recent-symbolic".to_owned()),
                        icon_path: None,
                        action: MenuAction::OpenSection("recent".to_owned()),
                    },
                    ButtonConfig {
                        id: "system".to_owned(),
                        label: "System".to_owned(),
                        icon_name: Some("applications-system-symbolic".to_owned()),
                        icon_path: None,
                        action: MenuAction::OpenSection("system".to_owned()),
                    },
                ],
                bottom_button: ButtonConfig {
                    id: "power".to_owned(),
                    label: "Power".to_owned(),
                    icon_name: Some("system-shutdown-symbolic".to_owned()),
                    icon_path: None,
                    action: MenuAction::None,
                },
            },
            header: HeaderBannerConfig {
                height: 212,
                spacing: 14,
                outer_margin: 24,
                title: "Start Menu".to_owned(),
                subtitle:
                    "Game-style header placeholder. This area can later map to profile, greeting, and system status."
                        .to_owned(),
                stats: vec![
                    HeaderStatConfig {
                        label: "Recent Apps".to_owned(),
                        value: "12".to_owned(),
                    },
                    HeaderStatConfig {
                        label: "Favorites".to_owned(),
                        value: "8".to_owned(),
                    },
                    HeaderStatConfig {
                        label: "System Status".to_owned(),
                        value: "Normal".to_owned(),
                    },
                ],
                action_buttons: vec![
                    ButtonConfig {
                        id: "edit".to_owned(),
                        label: "Edit".to_owned(),
                        icon_name: Some("document-edit-symbolic".to_owned()),
                        icon_path: None,
                        action: MenuAction::None,
                    },
                    ButtonConfig {
                        id: "notify".to_owned(),
                        label: "Alerts".to_owned(),
                        icon_name: Some("preferences-system-notifications-symbolic".to_owned()),
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
                    id: "applications".to_owned(),
                    label: "Applications".to_owned(),
                    icon_name: Some("view-app-grid-symbolic".to_owned()),
                    icon_path: None,
                    action: MenuAction::OpenSection("applications".to_owned()),
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
