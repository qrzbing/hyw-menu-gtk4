use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box as GtkBox, Button, Image, Justification, Label, Orientation,
    Overflow, gio,
};

use crate::app::catalog::DesktopAppEntry;

pub(crate) fn build_app_tile_widget(
    app: &DesktopAppEntry,
    tile_size: i32,
    icon_size: i32,
    icon_center_y_ratio: f32,
    title_font_size: i32,
    title_top_margin: i32,
    tooltip_fallback: Option<&str>,
) -> Button {
    let widget = Button::new();
    let content = GtkBox::new(Orientation::Vertical, 8);
    let icon_slot = GtkBox::new(Orientation::Vertical, 0);
    let icon = app
        .app_info()
        .icon()
        .map(|icon| Image::from_gicon(&icon))
        .unwrap_or_else(|| Image::from_icon_name("application-x-executable-symbolic"));
    let title = Label::new(Some(app.name()));
    let content_size = (tile_size - 16).max(40);
    let title_width = (tile_size - 28).max(24);
    let icon_slot_size = (icon_size + (tile_size / 12)).max(icon_size);
    let title_height_estimate = (title_font_size + 8).max(18);
    let desired_center_y = ((tile_size as f32) * icon_center_y_ratio).round() as i32;
    let desired_icon_top = desired_center_y - 8 - (icon_slot_size / 2);
    let max_icon_top =
        (content_size - icon_slot_size - title_top_margin - title_height_estimate).max(0);
    let icon_top_margin = desired_icon_top.clamp(0, max_icon_top);

    widget.add_css_class("launcher-app-tile");
    widget.set_size_request(tile_size, tile_size);
    widget.set_halign(Align::Start);
    widget.set_valign(Align::Start);
    widget.set_hexpand(false);
    widget.set_vexpand(false);
    widget.set_can_shrink(true);
    widget.set_overflow(Overflow::Hidden);
    widget.set_tooltip_text(Some(
        app.description()
            .filter(|description| !description.trim().is_empty())
            .or(tooltip_fallback)
            .unwrap_or(app.name()),
    ));

    icon_slot.add_css_class("launcher-app-tile-icon-slot");
    icon_slot.set_size_request(icon_slot_size, icon_slot_size);
    icon_slot.set_halign(Align::Center);
    icon_slot.set_valign(Align::Center);
    icon_slot.set_hexpand(false);
    icon_slot.set_vexpand(false);
    icon_slot.set_margin_top(icon_top_margin);

    icon.add_css_class("launcher-app-tile-icon");
    icon.set_pixel_size(icon_size);
    icon.set_halign(Align::Center);
    icon.set_valign(Align::Center);

    title.add_css_class("launcher-app-tile-title");
    title.set_halign(Align::Center);
    title.set_xalign(0.5);
    title.set_wrap(false);
    title.set_single_line_mode(true);
    title.set_ellipsize(EllipsizeMode::End);
    title.set_justify(Justification::Center);
    title.set_width_request(title_width);
    title.set_max_width_chars(1);
    title.set_hexpand(false);
    title.set_margin_top(title_top_margin);

    content.set_size_request(content_size, content_size);
    content.set_halign(Align::Center);
    content.set_valign(Align::Start);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.set_overflow(Overflow::Hidden);
    icon_slot.append(&icon);
    content.append(&icon_slot);
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
