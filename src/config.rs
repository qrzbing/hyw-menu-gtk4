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
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct HeaderBannerConfig {
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

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            application_id: "com.qrzbing.hyw-menu-gtk4".to_string(),
            window: WindowConfig {
                title: "hyw-menu".to_string(),
                namespace: "hyw-menu".to_string(),
                min_width: 320,
                sidebar_width: 88,
            },
            sidebar: SidebarConfig {
                spacing: 12,
                outer_margin: 24,
                inner_margin: 16,
                buttons: vec![
                    ButtonConfig {
                        id: "all".to_string(),
                        label: "All".to_string(),
                        icon_name: Some("view-grid-symbolic".to_string()),
                        action: MenuAction::OpenSection("all".to_string()),
                    },
                    ButtonConfig {
                        id: "favorites".to_string(),
                        label: "Favorites".to_string(),
                        icon_name: Some("starred-symbolic".to_string()),
                        action: MenuAction::OpenSection("favorites".to_string()),
                    },
                    ButtonConfig {
                        id: "recent".to_string(),
                        label: "Recent".to_string(),
                        icon_name: Some("document-open-recent-symbolic".to_string()),
                        action: MenuAction::OpenSection("recent".to_string()),
                    },
                    ButtonConfig {
                        id: "system".to_string(),
                        label: "System".to_string(),
                        icon_name: Some("applications-system-symbolic".to_string()),
                        action: MenuAction::OpenSection("system".to_string()),
                    },
                ],
            },
            header: HeaderBannerConfig {
                spacing: 14,
                outer_margin: 24,
                title: "Start Menu".to_string(),
                subtitle:
                    "Game-style header placeholder. This area can later map to profile, greeting, and system status.".to_string(),
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
                        action: MenuAction::None,
                    },
                    ButtonConfig {
                        id: "notify".to_string(),
                        label: "Alerts".to_string(),
                        icon_name: Some("preferences-system-notifications-symbolic".to_string()),
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
                    action: MenuAction::OpenSection("applications".to_string()),
                }],
            },
        }
    }
}
