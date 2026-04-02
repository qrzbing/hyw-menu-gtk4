use std::ffi::CString;
use std::os::raw::{c_int, c_uchar};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use gtk4::gdk::Display;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display,
};

use crate::config::LauncherConfig;

const STYLE_CSS: &str = include_str!("../assets/style.css");

#[repr(C)]
struct FcConfig {
    _private: [u8; 0],
}

#[link(name = "fontconfig")]
unsafe extern "C" {
    fn FcInit() -> c_int;
    fn FcConfigGetCurrent() -> *mut FcConfig;
    fn FcConfigAppFontAddFile(config: *mut FcConfig, file: *const c_uchar) -> c_int;
    fn FcConfigBuildFonts(config: *mut FcConfig) -> c_int;
}

pub fn install_global_css(display: &Display, config: &LauncherConfig) {
    register_theme_font(config);
    let provider = CssProvider::new();
    provider.load_from_string(&format!(
        "{STYLE_CSS}\n{}\n{}",
        sidebar_runtime_css(config),
        theme_runtime_css(config)
    ));
    style_context_add_provider_for_display(display, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn sidebar_runtime_css(config: &LauncherConfig) -> String {
    let opacity = config.sidebar().button_bg_opacity().clamp(0.0, 1.0);

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

fn theme_runtime_css(config: &LauncherConfig) -> String {
    let Some(font_family) = config.theme().font_family() else {
        return String::new();
    };

    let escaped_family = css_string_literal(font_family);
    format!("* {{ font-family: {escaped_family}, sans-serif; }}\n")
}

fn css_string_literal(input: &str) -> String {
    format!("\"{}\"", input.replace('\\', "\\\\").replace('"', "\\\""))
}

fn register_theme_font(config: &LauncherConfig) {
    let Some(font_path) = config.theme().font_path().filter(|path| path.exists()) else {
        return;
    };

    if let Err(error) = register_font_file(font_path) {
        eprintln!("failed to register font {}: {error}", font_path.display());
    }
}

fn register_font_file(path: &Path) -> Result<(), String> {
    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| format!("font path contains interior NUL: {}", path.display()))?;

    unsafe {
        if FcInit() == 0 {
            return Err("fcinit failed".to_owned());
        }

        let config = FcConfigGetCurrent();
        if config.is_null() {
            return Err("fcconfiggetcurrent returned null".to_owned());
        }

        if FcConfigAppFontAddFile(config, c_path.as_ptr() as *const c_uchar) == 0 {
            return Err("fcconfigappfontaddfile failed".to_owned());
        }

        if FcConfigBuildFonts(config) == 0 {
            return Err("fcconfigbuildfonts failed".to_owned());
        }
    }

    Ok(())
}
