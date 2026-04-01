#[derive(Debug, Clone)]
pub struct LauncherConfig {
    pub application_id: String,
    pub window: WindowConfig,
    pub sidebar: SidebarConfig,
    pub main_panel: MainPanelConfig,
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
pub struct MainPanelConfig {
    pub spacing: i32,
    pub outer_margin: i32,
    pub title: String,
    pub description: String,
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
                        label: "Fav".to_string(),
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
            main_panel: MainPanelConfig {
                spacing: 16,
                outer_margin: 24,
                title: "Start".to_string(),
                description: "Description".to_string(),
            },
        }
    }
}
