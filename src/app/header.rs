use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Stack};

use super::launcher::LauncherWindow;
use crate::config::{ButtonConfig, HeaderStatConfig};

impl LauncherWindow {
    pub(super) fn build_right_panel(&self, content_stack: &Stack) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, self.config().grid().spacing());
        panel.add_css_class("launcher-right-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        panel.set_margin_top(self.config().header().outer_margin());
        panel.set_margin_bottom(self.config().header().outer_margin());
        panel.set_margin_start(8);
        panel.set_margin_end(self.config().header().outer_margin());

        panel.append(&self.build_header_banner());
        panel.append(content_stack);
        panel
    }

    fn build_header_banner(&self) -> GtkBox {
        let banner = GtkBox::new(Orientation::Vertical, self.config().header().spacing());
        banner.set_hexpand(true);
        banner.set_height_request(self.config().header().height());
        banner.add_css_class("launcher-header-banner");

        let top_row = GtkBox::new(Orientation::Horizontal, self.config().header().spacing());
        let identity = GtkBox::new(Orientation::Horizontal, self.config().header().spacing());
        let text_column = GtkBox::new(Orientation::Vertical, 6);
        let actions = GtkBox::new(Orientation::Horizontal, 8);

        let avatar = Button::with_label("Avatar");
        avatar.set_width_request(84);
        avatar.set_height_request(84);
        avatar.add_css_class("header-avatar");

        let title = Label::new(Some(self.config().header().title()));
        title.set_halign(Align::Start);
        title.set_xalign(0.0);
        title.add_css_class("title-1");
        title.add_css_class("launcher-header-title");

        let subtitle = Label::new(Some(self.config().header().subtitle()));
        subtitle.set_halign(Align::Start);
        subtitle.set_xalign(0.0);
        subtitle.set_wrap(true);
        subtitle.add_css_class("launcher-header-subtitle");

        text_column.set_hexpand(true);
        text_column.append(&title);
        text_column.append(&subtitle);

        identity.set_hexpand(true);
        identity.append(&avatar);
        identity.append(&text_column);

        for button in self.config().header().action_buttons() {
            actions.append(&self.build_header_action_button(button));
        }

        top_row.append(&identity);
        top_row.append(&actions);

        let stats_row = GtkBox::new(Orientation::Horizontal, self.config().header().spacing());
        for stat in self.config().header().stats() {
            stats_row.append(&self.build_header_stat(stat));
        }

        banner.append(&top_row);
        banner.append(&stats_row);
        banner
    }

    fn build_header_action_button(&self, button: &ButtonConfig) -> Button {
        let widget = Button::with_label(button.label());
        widget.add_css_class("launcher-header-action-button");
        widget.set_tooltip_text(Some(&LauncherWindow::button_tooltip(button)));
        self.bind_button_action(&widget, button, None);
        widget
    }

    fn build_header_stat(&self, stat: &HeaderStatConfig) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 4);
        let label = Label::new(Some(stat.label()));
        let value = Label::new(Some(stat.value()));
        container.add_css_class("launcher-header-stat");
        label.add_css_class("launcher-header-stat-label");

        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        value.set_halign(Align::Start);
        value.set_xalign(0.0);
        value.add_css_class("title-4");
        value.add_css_class("launcher-header-stat-value");

        container.set_hexpand(true);
        container.append(&label);
        container.append(&value);
        container
    }
}
