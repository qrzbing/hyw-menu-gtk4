use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use gtk4::gdk::{self, Display};
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, ContentFit, Entry,
    EventControllerKey, FlowBox, Grid, Image, Label, Orientation, Picture, ScrolledWindow, Stack,
    StackTransitionType, Widget,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::config::{
    ButtonConfig, GridSectionConfig, HeaderStatConfig, LauncherConfig, MenuAction,
};
use crate::hyprland::{HyprlandContext, preferred_monitor};
use crate::style::install_global_css;

const ALL_APPS_PLACEHOLDERS: &[(&str, &str, &str)] = &[
    (
        "Terminal",
        "utilities-terminal-symbolic",
        "Command line and terminal tools",
    ),
    (
        "Browser",
        "web-browser-symbolic",
        "Web browsing and web apps",
    ),
    (
        "Files",
        "system-file-manager-symbolic",
        "File management and directory browsing",
    ),
    (
        "Settings",
        "preferences-system-symbolic",
        "System settings and device options",
    ),
    (
        "Editor",
        "accessories-text-editor-symbolic",
        "Code and text editing",
    ),
    (
        "Music",
        "multimedia-player-symbolic",
        "Music and audio playback",
    ),
    ("Video", "video-display-symbolic", "Video and display tools"),
    (
        "Archive",
        "package-x-generic-symbolic",
        "Archive and compression tools",
    ),
    (
        "Monitor",
        "utilities-system-monitor-symbolic",
        "System monitoring and resource usage",
    ),
    (
        "Network",
        "network-workgroup-symbolic",
        "Network and connectivity",
    ),
    (
        "Calendar",
        "x-office-calendar-symbolic",
        "Schedule and reminders",
    ),
    (
        "Image",
        "image-x-generic-symbolic",
        "Image viewing and organization",
    ),
];

#[derive(Clone)]
struct LauncherNavigator {
    stack: Stack,
    section_buttons: Rc<RefCell<Vec<(String, Button)>>>,
}

impl LauncherNavigator {
    fn new(stack: &Stack) -> Self {
        Self {
            stack: stack.clone(),
            section_buttons: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn register_sidebar_button(&self, section_id: &str, button: &Button) {
        self.section_buttons
            .borrow_mut()
            .push((section_id.to_string(), button.clone()));
    }

    fn activate_section(&self, section_id: &str) {
        self.stack
            .set_visible_child_name(Self::page_name_for_section(section_id));

        for (candidate_id, button) in self.section_buttons.borrow().iter() {
            if candidate_id == section_id {
                button.add_css_class("is-active");
            } else {
                button.remove_css_class("is-active");
            }
        }
    }

    fn page_name_for_section(section_id: &str) -> &'static str {
        match section_id {
            "home" | "dashboard" | "start" => "dashboard",
            _ => "all-apps",
        }
    }
}

pub struct LauncherApp {
    config: Arc<LauncherConfig>,
    hyprland: Option<HyprlandContext>,
}

impl LauncherApp {
    pub fn new(config: LauncherConfig) -> Self {
        Self {
            config: Arc::new(config),
            hyprland: HyprlandContext::new(),
        }
    }

    pub fn run(self) {
        let config = self.config;
        let hyprland = self.hyprland;
        let app = Application::builder()
            .application_id(&config.application_id)
            .build();

        app.connect_activate(move |application| {
            let display = Display::default().expect("No display available");
            install_global_css(&display, &config);
            let monitor = preferred_monitor(&display, hyprland.as_ref(), config.window.min_width)
                .expect("No monitor available");

            let launcher_window = LauncherWindow::new(application, &monitor, config.clone());
            launcher_window.present();
        });

        app.run_with_args::<&str>(&[]);
    }
}

pub struct LauncherWindow {
    window: ApplicationWindow,
    config: Arc<LauncherConfig>,
}

impl LauncherWindow {
    pub fn new(app: &Application, monitor: &gdk::Monitor, config: Arc<LauncherConfig>) -> Self {
        let geometry = monitor.geometry();
        let window = ApplicationWindow::builder()
            .application(app)
            .title(&config.window.title)
            .default_width(Self::preferred_width(monitor, &config))
            .default_height(geometry.height())
            .build();

        let launcher_window = Self { window, config };
        launcher_window.configure_layer_shell(monitor);
        launcher_window.install_keybindings();
        launcher_window.mount_content();
        launcher_window
    }

    pub fn present(&self) {
        self.window.present();
    }

    fn preferred_width(monitor: &gdk::Monitor, config: &LauncherConfig) -> i32 {
        (monitor.geometry().width() / 4).max(config.window.min_width)
    }

    fn mount_content(&self) {
        let root = self.build_root_container();
        self.window.set_child(Some(&root));
    }

    fn build_root_container(&self) -> GtkBox {
        let content_stack = Stack::new();
        let navigator = LauncherNavigator::new(&content_stack);
        self.populate_content_stack(&content_stack, &navigator);

        let root = GtkBox::new(Orientation::Horizontal, 0);
        root.add_css_class("launcher-root");
        root.append(&self.build_sidebar_nav(&navigator));
        root.append(&self.build_right_panel(&content_stack));
        navigator.activate_section(&self.initial_section_id());
        root
    }

    fn build_sidebar_nav(&self, navigator: &LauncherNavigator) -> GtkBox {
        let sidebar = GtkBox::new(Orientation::Vertical, 0);
        sidebar.add_css_class("launcher-sidebar");
        sidebar.set_width_request(self.sidebar_width());
        sidebar.set_margin_top(self.config.sidebar.outer_margin);
        sidebar.set_margin_bottom(self.config.sidebar.outer_margin);
        sidebar.set_margin_start(self.config.sidebar.inner_margin);
        sidebar.set_margin_end(self.config.sidebar.inner_margin);
        sidebar.set_valign(Align::Fill);
        sidebar.set_vexpand(true);

        let top_section = GtkBox::new(Orientation::Vertical, 0);
        top_section.add_css_class("launcher-sidebar-top");
        top_section.append(&self.build_sidebar_button(
            &self.config.sidebar.top_button,
            "launcher-sidebar-fixed-button",
            self.sidebar_button_size(),
            None,
        ));

        let middle_section = GtkBox::new(Orientation::Vertical, self.config.sidebar.spacing);
        middle_section.add_css_class("launcher-sidebar-middle");
        for button in &self.config.sidebar.buttons {
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
            &self.config.sidebar.bottom_button,
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
        widget.set_tooltip_text(Some(&Self::button_tooltip(button)));
        widget.set_child(Some(&icon_slot));

        if let (Some(navigator), MenuAction::OpenSection(section)) = (navigator, &button.action) {
            navigator.register_sidebar_button(section, &widget);
        }

        self.bind_button_action(&widget, button, navigator.cloned());
        widget
    }

    fn build_right_panel(&self, content_stack: &Stack) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, self.config.grid.spacing);
        panel.add_css_class("launcher-right-panel");
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        panel.set_margin_top(self.config.header.outer_margin);
        panel.set_margin_bottom(self.config.header.outer_margin);
        panel.set_margin_start(8);
        panel.set_margin_end(self.config.header.outer_margin);

        panel.append(&self.build_header_banner());
        panel.append(content_stack);
        panel
    }

    fn build_header_banner(&self) -> GtkBox {
        let banner = GtkBox::new(Orientation::Vertical, self.config.header.spacing);
        banner.set_hexpand(true);
        banner.set_height_request(self.config.header.height);
        banner.add_css_class("launcher-header-banner");

        let top_row = GtkBox::new(Orientation::Horizontal, self.config.header.spacing);
        let identity = GtkBox::new(Orientation::Horizontal, self.config.header.spacing);
        let text_column = GtkBox::new(Orientation::Vertical, 6);
        let actions = GtkBox::new(Orientation::Horizontal, 8);

        let avatar = Button::with_label("Avatar");
        avatar.set_width_request(84);
        avatar.set_height_request(84);
        avatar.add_css_class("header-avatar");

        let title = Label::new(Some(&self.config.header.title));
        title.set_halign(Align::Start);
        title.set_xalign(0.0);
        title.add_css_class("title-1");
        title.add_css_class("launcher-header-title");

        let subtitle = Label::new(Some(&self.config.header.subtitle));
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

        for button in &self.config.header.action_buttons {
            actions.append(&self.build_header_action_button(button));
        }

        top_row.append(&identity);
        top_row.append(&actions);

        let stats_row = GtkBox::new(Orientation::Horizontal, self.config.header.spacing);
        for stat in &self.config.header.stats {
            stats_row.append(&self.build_header_stat(stat));
        }

        banner.append(&top_row);
        banner.append(&stats_row);
        banner
    }

    fn build_header_action_button(&self, button: &ButtonConfig) -> Button {
        let widget = Button::with_label(&button.label);
        widget.add_css_class("launcher-header-action-button");
        widget.set_tooltip_text(Some(&Self::button_tooltip(button)));
        self.bind_button_action(&widget, button, None);
        widget
    }

    fn build_header_stat(&self, stat: &HeaderStatConfig) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 4);
        let label = Label::new(Some(&stat.label));
        let value = Label::new(Some(&stat.value));
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

    fn populate_content_stack(&self, content_stack: &Stack, navigator: &LauncherNavigator) {
        content_stack.add_css_class("launcher-content-stack");
        content_stack.set_hexpand(true);
        content_stack.set_vexpand(true);
        content_stack.set_transition_type(StackTransitionType::Crossfade);
        content_stack.add_named(&self.build_dashboard_page(navigator), Some("dashboard"));
        content_stack.add_named(&self.build_all_apps_page(), Some("all-apps"));
    }

    fn build_dashboard_page(&self, navigator: &LauncherNavigator) -> GtkBox {
        let page = GtkBox::new(Orientation::Vertical, 12);
        page.add_css_class("launcher-page");
        page.add_css_class("launcher-dashboard-page");
        page.set_hexpand(true);
        page.set_vexpand(true);
        page.append(&self.build_page_intro(
            "Quick Access",
            "Keep the current game-style home page as the entry view. Later it can host pinned apps, recent items, and system shortcuts.",
            "Home",
        ));
        page.append(&self.build_grid_section(navigator));
        page
    }

    fn build_all_apps_page(&self) -> GtkBox {
        let page = GtkBox::new(Orientation::Vertical, 12);
        page.add_css_class("launcher-page");
        page.add_css_class("launcher-all-apps-page");
        page.set_hexpand(true);
        page.set_vexpand(true);
        page.append(&self.build_page_intro(
            "All Apps",
            "This page follows the launcher structure of nwg-menu / nwg-drawer for now, using placeholder apps until desktop data is wired in.",
            "Menu",
        ));
        page.append(&self.build_all_apps_toolbar());
        page.append(&self.build_all_apps_scroller());
        page
    }

    fn build_page_intro(&self, title_text: &str, subtitle_text: &str, badge_text: &str) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 8);
        let top_row = GtkBox::new(Orientation::Horizontal, 10);
        let text_column = GtkBox::new(Orientation::Vertical, 4);

        let title = Label::new(Some(title_text));
        title.add_css_class("launcher-page-title");
        title.set_halign(Align::Start);
        title.set_xalign(0.0);

        let subtitle = Label::new(Some(subtitle_text));
        subtitle.add_css_class("launcher-page-subtitle");
        subtitle.set_wrap(true);
        subtitle.set_halign(Align::Start);
        subtitle.set_xalign(0.0);

        let badge = Label::new(Some(badge_text));
        badge.add_css_class("launcher-page-badge");
        badge.set_halign(Align::End);

        text_column.set_hexpand(true);
        text_column.append(&title);
        text_column.append(&subtitle);

        top_row.append(&text_column);
        top_row.append(&badge);

        container.add_css_class("launcher-page-intro");
        container.append(&top_row);
        container
    }

    fn build_all_apps_toolbar(&self) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 10);
        let search = Entry::new();
        let filters = GtkBox::new(Orientation::Horizontal, 8);

        search.add_css_class("launcher-search-entry");
        search.set_placeholder_text(Some(
            "Search apps (placeholder UI, desktop data will be added later)",
        ));

        for filter in ["All", "Pinned", "System", "Development"] {
            let chip = Button::with_label(filter);
            chip.add_css_class("launcher-filter-chip");
            filters.append(&chip);
        }

        container.add_css_class("launcher-all-apps-toolbar");
        container.append(&search);
        container.append(&filters);
        container
    }

    fn build_all_apps_scroller(&self) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();
        scroller.add_css_class("launcher-apps-scroller");

        let flow = FlowBox::new();
        let spacing = self.config.grid.spacing.max(0) as u32;
        flow.set_min_children_per_line(self.config.grid.columns.max(1) as u32);
        flow.set_max_children_per_line(self.config.grid.columns.max(1) as u32);
        flow.set_column_spacing(spacing);
        flow.set_row_spacing(spacing);
        flow.set_selection_mode(gtk4::SelectionMode::None);
        flow.set_homogeneous(true);
        flow.add_css_class("launcher-apps-grid");

        for (title, icon_name, subtitle) in ALL_APPS_PLACEHOLDERS {
            flow.insert(&self.build_all_apps_tile(title, icon_name, subtitle), -1);
        }

        scroller.set_child(Some(&flow));
        scroller
    }

    fn build_all_apps_tile(
        &self,
        title_text: &str,
        icon_name: &str,
        subtitle_text: &str,
    ) -> Button {
        let widget = Button::new();
        let content = GtkBox::new(Orientation::Vertical, 10);
        let icon = Image::from_icon_name(icon_name);
        let title = Label::new(Some(title_text));
        let subtitle = Label::new(Some(subtitle_text));

        widget.add_css_class("launcher-app-tile");
        widget.set_width_request(self.config.grid.tile_width + 18);
        widget.set_height_request(self.config.grid.tile_height + 26);
        widget.set_halign(Align::Fill);
        widget.set_valign(Align::Start);
        widget.set_tooltip_text(Some(title_text));

        icon.add_css_class("launcher-app-tile-icon");
        icon.set_pixel_size(28);
        icon.set_halign(Align::Start);

        title.add_css_class("launcher-app-tile-title");
        title.set_halign(Align::Start);
        title.set_xalign(0.0);

        subtitle.add_css_class("launcher-app-tile-subtitle");
        subtitle.set_wrap(true);
        subtitle.set_halign(Align::Start);
        subtitle.set_xalign(0.0);

        content.set_halign(Align::Fill);
        content.set_valign(Align::Center);
        content.append(&icon);
        content.append(&title);
        content.append(&subtitle);
        widget.set_child(Some(&content));
        widget
    }

    fn build_grid_section(&self, navigator: &LauncherNavigator) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();
        scroller.add_css_class("launcher-grid-scroller");

        let grid = Grid::new();
        let spacing = self.config.grid.spacing.max(0) as u32;
        grid.set_column_spacing(spacing);
        grid.set_row_spacing(spacing);
        grid.set_halign(Align::Start);
        grid.set_valign(Align::Start);
        grid.set_hexpand(false);
        grid.set_vexpand(false);
        grid.add_css_class("launcher-grid-section");

        for (index, button) in self.config.grid.buttons.iter().enumerate() {
            let column = (index as i32) % self.config.grid.columns;
            let row = (index as i32) / self.config.grid.columns;
            grid.attach(
                &self.build_menu_tile_button(button, &self.config.grid, Some(navigator)),
                column,
                row,
                1,
                1,
            );
        }

        scroller.set_child(Some(&grid));
        scroller
    }

    fn build_menu_tile_button(
        &self,
        button: &ButtonConfig,
        grid: &GridSectionConfig,
        navigator: Option<&LauncherNavigator>,
    ) -> Button {
        let widget = Button::new();
        let content = GtkBox::new(Orientation::Vertical, 8);
        let icon = Label::new(Some(button.icon_name.as_deref().unwrap_or("icon")));
        let title = Label::new(Some(&button.label));

        widget.set_width_request(grid.tile_width);
        widget.set_height_request(grid.tile_height);
        widget.set_halign(Align::Start);
        widget.set_valign(Align::Start);
        widget.set_hexpand(false);
        widget.set_vexpand(false);
        widget.set_tooltip_text(Some(&Self::button_tooltip(button)));
        widget.add_css_class("menu-tile-button");
        self.bind_button_action(&widget, button, navigator.cloned());

        content.set_valign(Align::Center);
        content.set_halign(Align::Center);
        content.set_hexpand(false);
        content.set_vexpand(false);

        icon.add_css_class("menu-tile-icon");
        icon.set_wrap(true);
        icon.set_justify(gtk4::Justification::Center);
        icon.set_max_width_chars(10);

        title.set_wrap(true);
        title.set_justify(gtk4::Justification::Center);
        title.set_max_width_chars(10);
        title.add_css_class("menu-tile-label");

        content.append(&icon);
        content.append(&title);
        widget.set_child(Some(&content));
        widget
    }

    fn build_sidebar_icon(&self, button: &ButtonConfig, icon_size: i32) -> Widget {
        if let Some(icon_path) = &button.icon_path {
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

        if let Some(icon_name) = &button.icon_name {
            let image = Image::from_icon_name(icon_name);
            image.add_css_class("launcher-sidebar-icon");
            image.set_halign(Align::Center);
            image.set_valign(Align::Center);
            image.set_width_request(icon_size);
            image.set_height_request(icon_size);
            image.set_pixel_size(icon_size);
            return image.upcast();
        }

        let fallback = Label::new(Some(&Self::sidebar_icon_fallback_text(button)));
        fallback.add_css_class("launcher-sidebar-icon");
        fallback.add_css_class("launcher-sidebar-icon-fallback");
        fallback.set_halign(Align::Center);
        fallback.set_valign(Align::Center);
        fallback.upcast()
    }

    fn bind_button_action(
        &self,
        widget: &Button,
        button: &ButtonConfig,
        navigator: Option<LauncherNavigator>,
    ) {
        let action = button.action.clone();
        let window = self.window.clone();

        widget.connect_clicked(move |_| match &action {
            MenuAction::CloseMenu => window.close(),
            MenuAction::OpenSection(section) => {
                if let Some(navigator) = &navigator {
                    navigator.activate_section(section);
                }
            }
            MenuAction::None | MenuAction::LaunchCommand(_) => {}
        });
    }

    fn sidebar_button_size(&self) -> i32 {
        ((self.sidebar_width() as f32) * self.config.sidebar.button_scale)
            .round()
            .clamp(40.0, self.sidebar_width() as f32) as i32
    }

    fn sidebar_width(&self) -> i32 {
        ((self.config.sidebar.width as f32) * self.config.sidebar.scale)
            .round()
            .max(56.0) as i32
    }

    fn sidebar_icon_fallback_text(button: &ButtonConfig) -> String {
        button
            .label
            .chars()
            .next()
            .map(|ch| ch.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string())
    }

    fn sidebar_middle_spacer_height(&self) -> i32 {
        (self.config.header.height + self.config.grid.spacing - self.sidebar_button_size()
            + self.config.sidebar.middle_offset)
            .max(0)
    }

    fn initial_section_id(&self) -> String {
        self.config
            .sidebar
            .buttons
            .iter()
            .find_map(|button| match &button.action {
                MenuAction::OpenSection(section) => Some(section.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "dashboard".to_string())
    }

    fn configure_layer_shell(&self, monitor: &gdk::Monitor) {
        self.window.set_decorated(false);
        self.window.set_resizable(false);

        self.window.init_layer_shell();
        self.window
            .set_namespace(Some(&self.config.window.namespace));
        self.window.set_layer(Layer::Overlay);
        self.window.set_keyboard_mode(KeyboardMode::Exclusive);
        self.window.set_monitor(Some(monitor));

        self.window.set_anchor(Edge::Left, true);
        self.window.set_anchor(Edge::Top, true);
        self.window.set_anchor(Edge::Bottom, true);
        self.window.set_anchor(Edge::Right, false);

        self.window.set_margin(Edge::Left, 0);
        self.window.set_margin(Edge::Top, 0);
        self.window.set_margin(Edge::Bottom, 0);
        self.window.set_margin(Edge::Right, 0);

        self.window.set_exclusive_zone(0);
    }

    fn install_keybindings(&self) {
        let key_controller = EventControllerKey::new();
        let window = self.window.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                window.close();
                Propagation::Stop
            } else {
                Propagation::Proceed
            }
        });

        self.window.add_controller(key_controller);
    }

    fn button_tooltip(button: &ButtonConfig) -> String {
        let icon = button.icon_name.as_deref().unwrap_or("none");

        match &button.action {
            MenuAction::None => format!("{} ({icon})", button.id),
            MenuAction::CloseMenu => format!("{} -> close ({icon})", button.id),
            MenuAction::OpenSection(section) => {
                format!("{} -> section:{section} ({icon})", button.id)
            }
            MenuAction::LaunchCommand(command) => {
                format!("{} -> command:{command} ({icon})", button.id)
            }
        }
    }
}
