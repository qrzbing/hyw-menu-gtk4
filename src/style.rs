use gtk4::gdk::Display;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, style_context_add_provider_for_display,
};

const STYLE_CSS: &str = include_str!("../assets/style.css");

pub fn install_global_css(display: &Display) {
    let provider = CssProvider::new();
    provider.load_from_string(STYLE_CSS);
    style_context_add_provider_for_display(display, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
}
