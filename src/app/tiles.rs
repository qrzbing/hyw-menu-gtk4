use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box as GtkBox, Button, Image, Justification, Label, Orientation, gio,
};

use crate::app::catalog::DesktopAppEntry;

pub(crate) fn build_app_tile_widget(
    app: &DesktopAppEntry,
    tile_size: i32,
    tooltip_fallback: Option<&str>,
) -> Button {
    let widget = Button::new();
    let content = GtkBox::new(Orientation::Vertical, 8);
    let icon = app
        .app_info()
        .icon()
        .map(|icon| Image::from_gicon(&icon))
        .unwrap_or_else(|| Image::from_icon_name("application-x-executable-symbolic"));
    let title = Label::new(Some(&truncate_app_title(app.name(), 12)));

    widget.add_css_class("launcher-app-tile");
    widget.set_width_request(tile_size);
    widget.set_height_request(tile_size);
    widget.set_halign(Align::Start);
    widget.set_valign(Align::Start);
    widget.set_hexpand(false);
    widget.set_vexpand(false);
    widget.set_tooltip_text(Some(
        app.description()
            .filter(|description| !description.trim().is_empty())
            .or(tooltip_fallback)
            .unwrap_or(app.name()),
    ));

    icon.add_css_class("launcher-app-tile-icon");
    icon.set_pixel_size(30);
    icon.set_halign(Align::Center);
    icon.set_valign(Align::Center);

    title.add_css_class("launcher-app-tile-title");
    title.set_halign(Align::Center);
    title.set_xalign(0.5);
    title.set_wrap(false);
    title.set_single_line_mode(true);
    title.set_justify(Justification::Center);
    title.set_width_chars(10);
    title.set_max_width_chars(10);

    content.set_halign(Align::Center);
    content.set_valign(Align::Center);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.append(&icon);
    content.append(&title);
    widget.set_child(Some(&content));

    widget
}

pub(crate) fn attach_launch_handler(
    widget: &Button,
    app: &DesktopAppEntry,
    window: &ApplicationWindow,
) {
    let app_info = app.app_info().clone();
    let app_name = app.name().to_owned();
    let window = window.clone();

    widget.connect_clicked(move |_| {
        if let Err(error) = app_info.launch(&[], None::<&gio::AppLaunchContext>) {
            eprintln!("failed to launch {app_name}: {error}");
            return;
        }

        window.close();
    });
}

fn truncate_app_title(input: &str, max_chars: usize) -> String {
    let mut chars = input.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}
