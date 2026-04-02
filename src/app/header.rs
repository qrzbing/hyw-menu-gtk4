use std::path::Path;

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, ContentFit, Label, Orientation, Overflow, Overlay, Picture, ProgressBar,
    SearchEntry, Stack, Widget,
};

use super::launcher::LauncherWindow;
use super::navigation::LauncherNavigator;
use super::pages::AllAppsPageState;
use crate::config::{
    AvatarPanelConfig, ProfileItemConfig, ProfilePanelConfig, ProfileProgressItemConfig,
    ProfileTextItemConfig, SearchPanelConfig, TextPanelConfig, TextPanelVariant, TopPanelConfig,
};

impl LauncherWindow {
    pub(super) fn build_right_panel(
        &self,
        content_stack: &Stack,
        navigator: &LauncherNavigator,
        all_apps: &AllAppsPageState,
    ) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, self.config().grid().spacing());
        panel.add_css_class("launcher-right-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        panel.set_margin_top(self.config().top_panels().outer_margin());
        panel.set_margin_bottom(self.config().top_panels().outer_margin());
        panel.set_margin_start(8);
        panel.set_margin_end(self.config().top_panels().outer_margin());

        if let Some(top_panels) = self.build_top_panels(navigator, all_apps) {
            panel.append(&top_panels);
        }

        panel.append(content_stack);
        panel
    }

    fn build_top_panels(
        &self,
        navigator: &LauncherNavigator,
        all_apps: &AllAppsPageState,
    ) -> Option<GtkBox> {
        if self.config().top_panels().left_panels().is_empty()
            && self.config().top_panels().right_panels().is_empty()
        {
            return None;
        }

        let container = GtkBox::new(
            Orientation::Horizontal,
            self.config().top_panels().spacing(),
        );
        container.add_css_class("launcher-top-banner");
        container.set_hexpand(true);
        container.set_height_request(self.config().top_panels().height());

        if !self.config().top_panels().left_panels().is_empty() {
            let left_column = self.build_top_panel_column(
                self.config().top_panels().left_panels(),
                navigator,
                all_apps,
                true,
            );
            container.append(&left_column);
        }

        if !self.config().top_panels().right_panels().is_empty() {
            let right_column = self.build_top_panel_column(
                self.config().top_panels().right_panels(),
                navigator,
                all_apps,
                self.config().top_panels().left_panels().is_empty(),
            );
            container.append(&right_column);
        }

        Some(container)
    }

    fn build_top_panel_column(
        &self,
        panels: &[TopPanelConfig],
        navigator: &LauncherNavigator,
        all_apps: &AllAppsPageState,
        is_primary: bool,
    ) -> GtkBox {
        let column = GtkBox::new(Orientation::Vertical, self.config().top_panels().spacing());
        let mut index = 0;

        column.add_css_class("launcher-top-panels-column");
        if is_primary {
            column.add_css_class("launcher-top-panels-column-primary");
            column.set_hexpand(true);
        } else {
            column.add_css_class("launcher-top-panels-column-secondary");
            column.set_width_request(280);
        }

        while index < panels.len() {
            column.append(&self.build_top_panel_widget(&panels[index], navigator, all_apps));
            index += 1;
        }

        column
    }

    fn build_top_panel_widget(
        &self,
        panel: &TopPanelConfig,
        navigator: &LauncherNavigator,
        all_apps: &AllAppsPageState,
    ) -> Widget {
        match panel {
            TopPanelConfig::Avatar(config) => self.build_avatar_panel(config).upcast(),
            TopPanelConfig::Profile(config) => self.build_profile_panel(config).upcast(),
            TopPanelConfig::Text(config) => self.build_text_panel(config).upcast(),
            TopPanelConfig::Search(config) => self
                .build_search_panel(config, navigator, all_apps)
                .upcast(),
        }
    }

    fn build_avatar_panel(&self, config: &AvatarPanelConfig) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 0);

        panel.add_css_class("launcher-top-panel");
        panel.add_css_class("launcher-avatar-panel");
        panel.set_width_request(config.size() + 8);
        panel.set_vexpand(false);
        panel.set_valign(Align::Start);
        panel.append(&self.build_avatar_frame(config.image_path(), config.label(), config.size()));
        panel
    }

    fn build_profile_panel(&self, config: &ProfilePanelConfig) -> Overlay {
        let panel = Overlay::new();
        let content = GtkBox::new(Orientation::Horizontal, 18);
        let sidebar = GtkBox::new(Orientation::Vertical, 10);
        let body = GtkBox::new(Orientation::Vertical, 10);

        panel.add_css_class("launcher-top-panel");
        panel.add_css_class("launcher-profile-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(false);
        panel.set_valign(Align::Start);
        panel.set_overflow(Overflow::Hidden);
        content.add_css_class("launcher-profile-panel-content");
        content.set_hexpand(true);
        content.set_vexpand(false);
        sidebar.add_css_class("launcher-profile-sidebar");
        sidebar.set_width_request((config.avatar_size() + 36).max(128));
        sidebar.set_vexpand(false);
        body.add_css_class("launcher-profile-body");
        body.set_hexpand(true);

        if let Some(background_path) = config.background_path() {
            let picture = Picture::for_filename(background_path);
            picture.add_css_class("launcher-profile-background");
            picture.set_can_shrink(true);
            picture.set_content_fit(ContentFit::Cover);
            picture.set_halign(Align::Fill);
            picture.set_valign(Align::Fill);
            panel.set_child(Some(&picture));
        }

        sidebar.append(&self.build_avatar_frame(
            config.avatar_path(),
            config.avatar_label(),
            config.avatar_size(),
        ));

        if let Some(uid) = config.uid() {
            let uid_label = Label::new(Some(uid));
            uid_label.add_css_class("launcher-profile-uid");
            uid_label.set_halign(Align::Center);
            uid_label.set_xalign(0.5);
            sidebar.append(&uid_label);
        }

        if let Some(action_text) = config.action_text() {
            let action_label = Label::new(Some(action_text));
            action_label.add_css_class("launcher-profile-action-text");
            action_label.set_halign(Align::Center);
            action_label.set_xalign(0.5);
            sidebar.append(&action_label);
        }

        let title_label = Label::new(Some(config.title()));
        title_label.add_css_class("launcher-profile-title");
        title_label.set_halign(Align::Start);
        title_label.set_xalign(0.0);
        body.append(&title_label);

        if let Some(subtitle) = config.subtitle() {
            let subtitle_label = Label::new(Some(subtitle));
            subtitle_label.add_css_class("launcher-profile-subtitle");
            subtitle_label.set_halign(Align::Start);
            subtitle_label.set_xalign(0.0);
            subtitle_label.set_wrap(true);
            subtitle_label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
            subtitle_label.set_lines(2);
            subtitle_label.set_ellipsize(EllipsizeMode::End);
            body.append(&subtitle_label);
        }

        if !config.items().is_empty() {
            let items_box = GtkBox::new(Orientation::Vertical, 8);
            items_box.add_css_class("launcher-profile-items");

            for item in config.items() {
                items_box.append(&self.build_profile_item(item));
            }

            body.append(&items_box);
        }

        content.append(&sidebar);
        content.append(&body);
        panel.add_overlay(&content);
        panel
    }

    fn build_profile_item(&self, item: &ProfileItemConfig) -> GtkBox {
        match item {
            ProfileItemConfig::Text(config) => self.build_profile_text_item(config),
            ProfileItemConfig::Progress(config) => self.build_profile_progress_item(config),
        }
    }

    fn build_profile_text_item(&self, config: &ProfileTextItemConfig) -> GtkBox {
        let row = GtkBox::new(Orientation::Horizontal, 10);
        let label = Label::new(Some(config.label()));
        let value = Label::new(Some(config.value()));

        row.add_css_class("launcher-profile-item");
        row.add_css_class("launcher-profile-item-text");
        row.set_hexpand(true);

        label.add_css_class("launcher-profile-item-label");
        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        value.add_css_class("launcher-profile-item-value");
        value.set_halign(Align::End);
        value.set_hexpand(true);
        value.set_xalign(1.0);

        row.append(&label);
        row.append(&value);
        row
    }

    fn build_profile_progress_item(&self, config: &ProfileProgressItemConfig) -> GtkBox {
        let item = GtkBox::new(Orientation::Vertical, 6);
        let header = GtkBox::new(Orientation::Horizontal, 10);
        let label = Label::new(Some(config.label()));
        let value = Label::new(Some(config.value()));
        let progress = ProgressBar::new();

        item.add_css_class("launcher-profile-item");
        item.add_css_class("launcher-profile-item-progress");
        item.set_hexpand(true);
        header.set_hexpand(true);

        label.add_css_class("launcher-profile-item-label");
        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        value.add_css_class("launcher-profile-item-value");
        value.set_halign(Align::End);
        value.set_hexpand(true);
        value.set_xalign(1.0);

        progress.add_css_class("launcher-profile-progress-bar");
        progress.set_hexpand(true);
        progress.set_fraction(config.progress());
        progress.set_show_text(false);

        header.append(&label);
        header.append(&value);
        item.append(&header);
        item.append(&progress);
        item
    }

    fn build_text_panel(&self, config: &TextPanelConfig) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 8);

        panel.add_css_class("launcher-top-panel");
        panel.add_css_class("launcher-text-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(false);
        panel.set_valign(Align::Start);
        panel.set_height_request(config.min_height());

        if config.variant() == TextPanelVariant::Hero {
            panel.add_css_class("launcher-text-panel-hero");
        } else {
            panel.add_css_class("launcher-text-panel-card");
        }

        if let Some(badge) = config.badge() {
            let badge_label = Label::new(Some(badge));
            badge_label.add_css_class("launcher-text-panel-badge");
            badge_label.set_halign(Align::Start);
            badge_label.set_xalign(0.0);
            panel.append(&badge_label);
        }

        if let Some(title) = config.title() {
            let title_label = Label::new(Some(title));
            title_label.set_halign(Align::Start);
            title_label.set_xalign(0.0);
            title_label.set_wrap(true);
            title_label.add_css_class("launcher-text-panel-title");
            if config.variant() == TextPanelVariant::Hero {
                title_label.add_css_class("launcher-text-panel-title-hero");
            }
            panel.append(&title_label);
        }

        if let Some(body) = config.body() {
            let body_label = Label::new(Some(body));
            body_label.set_halign(Align::Start);
            body_label.set_xalign(0.0);
            body_label.set_wrap(true);
            body_label.add_css_class("launcher-text-panel-body");
            panel.append(&body_label);
        }

        panel
    }

    fn build_search_panel(
        &self,
        config: &SearchPanelConfig,
        navigator: &LauncherNavigator,
        all_apps: &AllAppsPageState,
    ) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, 8);
        let search = SearchEntry::new();

        panel.add_css_class("launcher-top-panel");
        panel.add_css_class("launcher-search-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(false);
        panel.set_valign(Align::Start);
        panel.set_height_request(config.min_height());

        if let Some(title) = config.title() {
            let title_label = Label::new(Some(title));
            title_label.add_css_class("launcher-search-panel-title");
            title_label.set_halign(Align::Start);
            title_label.set_xalign(0.0);
            panel.append(&title_label);
        }

        search.add_css_class("launcher-search-entry");
        search.set_hexpand(true);
        search.set_width_chars(24);
        search.set_placeholder_text(Some(config.placeholder()));

        let navigator = navigator.clone();
        let all_apps = all_apps.clone();
        search.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            all_apps.set_query(&text);
            if !text.trim().is_empty() {
                navigator.activate_section("all");
            }
        });

        panel.append(&search);
        panel
    }

    fn avatar_fallback_text(&self, label: &str) -> String {
        label
            .chars()
            .next()
            .map(|ch| ch.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_owned())
    }

    fn build_avatar_frame(&self, image_path: Option<&Path>, label: &str, size: i32) -> GtkBox {
        let frame = GtkBox::new(Orientation::Vertical, 0);

        frame.add_css_class("launcher-avatar-frame");
        frame.set_halign(Align::Start);
        frame.set_valign(Align::Start);
        frame.set_width_request(size);
        frame.set_height_request(size);
        frame.set_overflow(Overflow::Hidden);

        if let Some(image_path) = image_path {
            let picture = Picture::for_filename(image_path);
            picture.add_css_class("launcher-avatar-picture");
            picture.set_can_shrink(true);
            picture.set_content_fit(ContentFit::Cover);
            picture.set_width_request(size);
            picture.set_height_request(size);
            frame.append(&picture);
        } else {
            let fallback = Label::new(Some(&self.avatar_fallback_text(label)));
            fallback.add_css_class("launcher-avatar-fallback");
            fallback.set_halign(Align::Center);
            fallback.set_valign(Align::Center);
            frame.append(&fallback);
        }

        frame
    }
}
