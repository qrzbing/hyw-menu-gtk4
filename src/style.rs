use gtk4::gdk::Display;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display,
};

use crate::config::LauncherConfig;

const STYLE_CSS: &str = include_str!("../assets/style.css");

pub fn install_global_css(display: &Display, config: &LauncherConfig) {
    let provider = CssProvider::new();
    provider.load_from_string(&format!("{STYLE_CSS}\n{}", sidebar_runtime_css(config)));
    style_context_add_provider_for_display(display, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn sidebar_runtime_css(config: &LauncherConfig) -> String {
    let opacity = config.sidebar.button_bg_opacity.clamp(0.0, 1.0);

    format!(
        r#"
.launcher-sidebar-button {{
  background: rgba(255, 248, 235, {base_bg});
  border-color: rgba(255, 245, 226, {base_border});
}}

.launcher-sidebar-fixed-button {{
  background: rgba(255, 248, 235, {fixed_bg});
  border-color: rgba(255, 239, 214, {fixed_border});
}}

.launcher-sidebar-menu-button {{
  background: rgba(255, 248, 235, {menu_bg});
}}

.launcher-sidebar-button:hover {{
  background: rgba(255, 248, 235, {hover_bg});
}}

.launcher-sidebar-button:checked,
.launcher-sidebar-button:active,
.launcher-sidebar-button.is-active,
.launcher-sidebar-button:focus-visible {{
  background: rgba(245, 214, 148, {active_bg});
  border-color: rgba(245, 214, 148, {active_border});
}}
"#,
        base_bg = 0.04_f32 * opacity,
        base_border = 0.06_f32 * opacity,
        fixed_bg = 0.12_f32 * opacity,
        fixed_border = 0.16_f32 * opacity,
        menu_bg = 0.08_f32 * opacity,
        hover_bg = 0.16_f32 * opacity,
        active_bg = 0.22_f32 * opacity,
        active_border = 0.5_f32 * opacity,
    )
}
