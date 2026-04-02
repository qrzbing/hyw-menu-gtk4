use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::error::ConfigError;

#[derive(Debug, Deserialize, Default)]
pub(super) struct LauncherFileConfig {
    #[serde(default)]
    pub(super) theme: ThemeFileConfig,
    #[serde(default)]
    pub(super) window: WindowFileConfig,
    #[serde(default)]
    pub(super) sidebar: SidebarFileConfig,
    #[serde(default)]
    pub(super) top_panels: TopPanelsFileConfig,
    #[serde(default)]
    pub(super) grid: GridFileConfig,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct ThemeFileConfig {
    pub(super) font_family: Option<String>,
    pub(super) font_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct WindowFileConfig {
    pub(super) min_width: Option<i32>,
    pub(super) width_ratio: Option<f32>,
    pub(super) height_ratio: Option<f32>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct SidebarFileConfig {
    pub(super) width: Option<i32>,
    pub(super) scale: Option<f32>,
    pub(super) button_scale: Option<f32>,
    pub(super) button_bg_opacity: Option<f32>,
    #[serde(default)]
    pub(super) top_button: Option<ButtonFileConfig>,
    #[serde(default)]
    pub(super) buttons: Vec<SidebarButtonFileEntry>,
    #[serde(default)]
    pub(super) bottom_button: Option<ButtonFileConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct GridFileConfig {
    pub(super) tile_size: Option<i32>,
    pub(super) tile_width: Option<i32>,
    pub(super) tile_height: Option<i32>,
    #[serde(default)]
    pub(super) cards: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct TopPanelsFileConfig {
    pub(super) height: Option<i32>,
    pub(super) spacing: Option<i32>,
    pub(super) outer_margin: Option<i32>,
    #[serde(default)]
    pub(super) left: Vec<TopPanelFileConfig>,
    #[serde(default)]
    pub(super) right: Vec<TopPanelFileConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum TopPanelFileConfig {
    Avatar(AvatarPanelFileConfig),
    Text(TextPanelFileConfig),
    Search(SearchPanelFileConfig),
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct AvatarPanelFileConfig {
    #[serde(alias = "image_path")]
    pub(super) image: Option<PathBuf>,
    pub(super) label: Option<String>,
    pub(super) size: Option<i32>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct TextPanelFileConfig {
    pub(super) variant: Option<String>,
    pub(super) title: Option<String>,
    pub(super) body: Option<String>,
    pub(super) badge: Option<String>,
    pub(super) min_height: Option<i32>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct SearchPanelFileConfig {
    pub(super) title: Option<String>,
    pub(super) placeholder: Option<String>,
    pub(super) min_height: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum SidebarButtonFileEntry {
    Label(String),
    Config(ButtonFileConfig),
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct ButtonFileConfig {
    pub(super) id: Option<String>,
    #[serde(alias = "name")]
    pub(super) label: Option<String>,
    pub(super) icon: Option<String>,
    #[serde(alias = "icon_name")]
    pub(super) icon_name: Option<String>,
    #[serde(alias = "icon_path")]
    pub(super) icon_path: Option<PathBuf>,
}

impl LauncherFileConfig {
    pub(super) fn from_path(path: &Path) -> Result<Self, ConfigError> {
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
