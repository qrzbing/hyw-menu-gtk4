use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, ContentFit, Image, Label, Orientation, Picture, Widget};

use super::launcher::LauncherWindow;
use super::navigation::LauncherNavigator;
use crate::config::{ButtonConfig, MenuAction};

impl LauncherWindow {
    pub(super) fn build_sidebar_nav(&self, navigator: &LauncherNavigator) -> GtkBox {
        let sidebar = GtkBox::new(Orientation::Vertical, 0);
        sidebar.add_css_class("launcher-sidebar");
        sidebar.set_width_request(self.sidebar_width());
        sidebar.set_margin_top(self.config().sidebar().outer_margin());
        sidebar.set_margin_bottom(self.config().sidebar().outer_margin());
        sidebar.set_margin_start(self.config().sidebar().inner_margin());
        sidebar.set_margin_end(self.config().sidebar().inner_margin());
        sidebar.set_valign(Align::Fill);
        sidebar.set_vexpand(true);

        let top_section = GtkBox::new(Orientation::Vertical, 0);
        top_section.add_css_class("launcher-sidebar-top");
        top_section.append(&self.build_sidebar_button(
            self.config().sidebar().top_button(),
            "launcher-sidebar-fixed-button",
            self.sidebar_button_size(),
            None,
        ));

        let middle_section = GtkBox::new(Orientation::Vertical, self.config().sidebar().spacing());
        middle_section.add_css_class("launcher-sidebar-middle");
        for button in self.config().sidebar().buttons() {
            middle_section.append(&self.build_sidebar_button(
                button,
                "launcher-sidebar-menu-button",
                self.sidebar_button_size(),
                Some(navigator),
            ));
        }

        let align_spacer = GtkBox::new(Orientation::Vertical, 0);
        align_spacer.set_height_request(self.sidebar_middle_spacer_height());

        let bottom_spacer = GtkBox::new(Orientation::Vertical, 0);
        bottom_spacer.set_vexpand(true);

        let bottom_section = GtkBox::new(Orientation::Vertical, 0);
        bottom_section.add_css_class("launcher-sidebar-bottom");
        bottom_section.append(&self.build_sidebar_button(
            self.config().sidebar().bottom_button(),
            "launcher-sidebar-fixed-button",
            self.sidebar_button_size(),
            None,
        ));

        sidebar.append(&top_section);
        sidebar.append(&align_spacer);
        sidebar.append(&middle_section);
        sidebar.append(&bottom_spacer);
        sidebar.append(&bottom_section);
        sidebar
    }

    fn build_sidebar_button(
        &self,
        button: &ButtonConfig,
        variant_class: &str,
        button_size: i32,
        navigator: Option<&LauncherNavigator>,
    ) -> Button {
        let widget = Button::new();
        let icon_slot = GtkBox::new(Orientation::Vertical, 0);
        widget.add_css_class("launcher-sidebar-button");
        widget.add_css_class(variant_class);
        icon_slot.add_css_class("launcher-sidebar-icon-slot");
        icon_slot.set_halign(Align::Center);
        icon_slot.set_valign(Align::Center);
        icon_slot.set_hexpand(false);
        icon_slot.set_vexpand(false);
        icon_slot.set_width_request(button_size);
        icon_slot.set_height_request(button_size);

        icon_slot.append(&self.build_sidebar_icon(button, button_size));

        widget.set_halign(Align::Center);
        widget.set_valign(Align::Start);
        widget.set_hexpand(false);
        widget.set_width_request(button_size);
        widget.set_height_request(button_size);
        widget.set_tooltip_text(Some(&LauncherWindow::button_tooltip(button)));
        widget.set_child(Some(&icon_slot));

        if let (Some(navigator), MenuAction::OpenSection(section)) = (navigator, button.action()) {
            navigator.register_sidebar_button(section, &widget);
        }

        self.bind_button_action(&widget, button, navigator.cloned());
        widget
    }

    fn build_sidebar_icon(&self, button: &ButtonConfig, icon_size: i32) -> Widget {
        if let Some(icon_path) = button.icon_path() {
            let picture = Picture::for_filename(icon_path);
            picture.add_css_class("launcher-sidebar-icon");
            picture.set_can_shrink(true);
            picture.set_content_fit(ContentFit::Contain);
            picture.set_halign(Align::Center);
            picture.set_valign(Align::Center);
            picture.set_width_request(icon_size);
            picture.set_height_request(icon_size);
            return picture.upcast();
        }

        if let Some(icon_name) = button.icon_name() {
            let image = Image::from_icon_name(icon_name);
            image.add_css_class("launcher-sidebar-icon");
            image.set_halign(Align::Center);
            image.set_valign(Align::Center);
            image.set_width_request(icon_size);
            image.set_height_request(icon_size);
            image.set_pixel_size(icon_size);
            return image.upcast();
        }

        let fallback = Label::new(Some(&LauncherWindow::sidebar_icon_fallback_text(button)));
        fallback.add_css_class("launcher-sidebar-icon");
        fallback.add_css_class("launcher-sidebar-icon-fallback");
        fallback.set_halign(Align::Center);
        fallback.set_valign(Align::Center);
        fallback.upcast()
    }
}
