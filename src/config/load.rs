use std::env;
use std::path::{Path, PathBuf};

use super::file::{ButtonFileConfig, LauncherFileConfig, SidebarButtonFileEntry};
use super::model::{
    ButtonConfig, GridSectionConfig, HeaderBannerConfig, HeaderStatConfig, LauncherConfig,
    MenuAction, SidebarConfig, SidebarMetrics, SidebarSizing, SidebarSpacing, ThemeConfig,
    WindowConfig,
};

impl LauncherConfig {
    pub fn load(config_path: Option<PathBuf>) -> Result<Self, super::ConfigError> {
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
            self.theme_mut()
                .set_font_family((!trimmed.is_empty()).then(|| trimmed.to_owned()));
        }

        if let Some(font_path) = resolve_config_path(config_dir, file_config.theme.font_path) {
            self.theme_mut().set_font_path(Some(font_path));
        }

        if let Some(min_width) = file_config.window.min_width {
            self.window_mut().set_min_width(min_width.max(320));
        }

        if let Some(width_ratio) = file_config.window.width_ratio {
            self.window_mut()
                .set_width_ratio(width_ratio.clamp(0.25, 0.5));
        }

        if let Some(height_ratio) = file_config.window.height_ratio {
            self.window_mut()
                .set_height_ratio(height_ratio.clamp(0.5, 0.95));
        }

        if let Some(width) = file_config.sidebar.width {
            self.sidebar_mut().set_width(width.max(56));
        }

        if let Some(scale) = file_config.sidebar.scale {
            self.sidebar_mut().set_scale(scale.clamp(0.5, 2.0));
        }

        if let Some(button_scale) = file_config.sidebar.button_scale {
            self.sidebar_mut()
                .set_button_scale(button_scale.clamp(0.4, 1.0));
        }

        if let Some(button_bg_opacity) = file_config.sidebar.button_bg_opacity {
            self.sidebar_mut()
                .set_button_bg_opacity(button_bg_opacity.clamp(0.0, 1.0));
        }

        if let Some(top_button) = file_config.sidebar.top_button {
            apply_button_file_config(self.sidebar_mut().top_button_mut(), top_button, config_dir);
        }

        if !file_config.sidebar.buttons.is_empty() {
            let buttons = file_config
                .sidebar
                .buttons
                .into_iter()
                .map(|button| sidebar_button_config_from_file(button, config_dir))
                .collect();
            self.sidebar_mut().set_buttons(buttons);
        }

        if let Some(bottom_button) = file_config.sidebar.bottom_button {
            apply_button_file_config(
                self.sidebar_mut().bottom_button_mut(),
                bottom_button,
                config_dir,
            );
        }

        if !file_config.grid.cards.is_empty() {
            let buttons = file_config
                .grid
                .cards
                .into_iter()
                .map(|label| grid_button_config(&label))
                .collect();
            self.grid_mut().set_buttons(buttons);
        }

        let legacy_tile_size = file_config.grid.tile_width.or(file_config.grid.tile_height);
        if let Some(tile_size) = file_config.grid.tile_size.or(legacy_tile_size) {
            self.grid_mut().set_tile_size(tile_size.clamp(72, 192));
        }
    }
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self::new(
            "com.qrzbing.hyw-menu-gtk4".to_owned(),
            default_config_dir().join("quick-access.toml"),
            ThemeConfig::new(None, None),
            WindowConfig::new("hyw-menu".to_owned(), "hyw-menu".to_owned(), 560, 0.34, 0.82),
            SidebarConfig::new(
                SidebarMetrics::new(
                    SidebarSizing::new(96, 1.0, 0.82, 1.0),
                    SidebarSpacing::new(12, 24, 16, 40),
                ),
                ButtonConfig::new(
                    "close-menu".to_owned(),
                    "Close".to_owned(),
                    Some("window-close-symbolic".to_owned()),
                    None,
                    MenuAction::CloseMenu,
                ),
                vec![
                    ButtonConfig::new(
                        "all".to_owned(),
                        "All".to_owned(),
                        Some("view-grid-symbolic".to_owned()),
                        None,
                        MenuAction::OpenSection("all".to_owned()),
                    ),
                    ButtonConfig::new(
                        "favorites".to_owned(),
                        "Favorites".to_owned(),
                        Some("starred-symbolic".to_owned()),
                        None,
                        MenuAction::OpenSection("favorites".to_owned()),
                    ),
                    ButtonConfig::new(
                        "recent".to_owned(),
                        "Recent".to_owned(),
                        Some("document-open-recent-symbolic".to_owned()),
                        None,
                        MenuAction::OpenSection("recent".to_owned()),
                    ),
                    ButtonConfig::new(
                        "system".to_owned(),
                        "System".to_owned(),
                        Some("applications-system-symbolic".to_owned()),
                        None,
                        MenuAction::OpenSection("system".to_owned()),
                    ),
                ],
                ButtonConfig::new(
                    "power".to_owned(),
                    "Power".to_owned(),
                    Some("system-shutdown-symbolic".to_owned()),
                    None,
                    MenuAction::None,
                ),
            ),
            HeaderBannerConfig::new(
                212,
                14,
                24,
                "Start Menu".to_owned(),
                "Game-style header placeholder. This area can later map to profile, greeting, and system status."
                    .to_owned(),
                vec![
                    HeaderStatConfig::new("Recent Apps".to_owned(), "12".to_owned()),
                    HeaderStatConfig::new("Favorites".to_owned(), "8".to_owned()),
                    HeaderStatConfig::new("System Status".to_owned(), "Normal".to_owned()),
                ],
                vec![
                    ButtonConfig::new(
                        "edit".to_owned(),
                        "Edit".to_owned(),
                        Some("document-edit-symbolic".to_owned()),
                        None,
                        MenuAction::None,
                    ),
                    ButtonConfig::new(
                        "notify".to_owned(),
                        "Alerts".to_owned(),
                        Some("preferences-system-notifications-symbolic".to_owned()),
                        None,
                        MenuAction::None,
                    ),
                ],
            ),
            GridSectionConfig::new(
                12,
                4,
                96,
                vec![ButtonConfig::new(
                    "applications".to_owned(),
                    "Applications".to_owned(),
                    Some("view-app-grid-symbolic".to_owned()),
                    None,
                    MenuAction::OpenSection("applications".to_owned()),
                )],
            ),
        )
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

    ButtonConfig::new(
        id.clone(),
        label.to_owned(),
        None,
        None,
        MenuAction::OpenSection(id),
    )
}

fn grid_button_config(label: &str) -> ButtonConfig {
    let id = slugify(label);

    ButtonConfig::new(
        id.clone(),
        label.to_owned(),
        None,
        None,
        MenuAction::OpenSection(id),
    )
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
            let label = default_text.unwrap_or_else(|| "Button".to_owned());
            let (icon_name, icon_path) = resolve_button_icon_fields(&button, config_dir);
            let id = button.id.unwrap_or_else(|| slugify(&label));

            ButtonConfig::new(
                id.clone(),
                label,
                icon_name,
                icon_path,
                MenuAction::OpenSection(id),
            )
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
