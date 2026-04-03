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
    pub(super) character_video: CharacterVideoFileConfig,
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
    pub(super) max_width: Option<i32>,
    pub(super) width_ratio: Option<f32>,
    pub(super) height_ratio: Option<f32>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct CharacterVideoFileConfig {
    pub(super) enabled: Option<bool>,
    pub(super) path: Option<PathBuf>,
    pub(super) height_ratio: Option<f32>,
    pub(super) offset_x: Option<i32>,
    pub(super) offset_y: Option<i32>,
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
    pub(super) tile_spacing: Option<i32>,
    pub(super) tile_size: Option<i32>,
    pub(super) side_margin: Option<i32>,
    pub(super) icon_size: Option<i32>,
    pub(super) icon_center_y_ratio: Option<f32>,
    pub(super) title_font_size: Option<i32>,
    pub(super) title_top_margin: Option<i32>,
    pub(super) tile_width: Option<i32>,
    pub(super) tile_height: Option<i32>,
    #[serde(default)]
    pub(super) cards: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct TopPanelsFileConfig {
    pub(super) height: Option<i32>,
    pub(super) spacing: Option<i32>,
    pub(super) top_margin: Option<i32>,
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
    Profile(ProfilePanelFileConfig),
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
pub(super) struct ProfilePanelFileConfig {
    #[serde(alias = "background")]
    pub(super) background_path: Option<PathBuf>,
    #[serde(alias = "avatar")]
    pub(super) avatar_path: Option<PathBuf>,
    pub(super) avatar_label: Option<String>,
    pub(super) avatar_size: Option<i32>,
    pub(super) uid: Option<String>,
    pub(super) action_text: Option<String>,
    pub(super) title: Option<String>,
    pub(super) subtitle: Option<String>,
    #[serde(default)]
    pub(super) items: Vec<ProfileItemFileConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum ProfileItemFileConfig {
    Text(ProfileTextItemFileConfig),
    Progress(ProfileProgressItemFileConfig),
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct ProfileTextItemFileConfig {
    pub(super) label: Option<String>,
    pub(super) value: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct ProfileProgressItemFileConfig {
    pub(super) label: Option<String>,
    pub(super) value: Option<String>,
    pub(super) progress: Option<f64>,
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
