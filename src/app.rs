use std::sync::Arc;

use gtk4::gdk::{self, Display};
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Grid, Label,
    Orientation, ScrolledWindow,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::config::{
    ButtonConfig, GridSectionConfig, HeaderStatConfig, LauncherConfig, MenuAction,
};
use crate::hyprland::{HyprlandContext, preferred_monitor};

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
        let root = GtkBox::new(Orientation::Horizontal, 0);
        root.append(&self.build_sidebar_nav());
        root.append(&self.build_right_panel());
        root
    }

    fn build_sidebar_nav(&self) -> GtkBox {
        let sidebar = GtkBox::new(Orientation::Vertical, self.config.sidebar.spacing);
        sidebar.set_width_request(self.config.window.sidebar_width);
        sidebar.set_margin_top(self.config.sidebar.outer_margin);
        sidebar.set_margin_bottom(self.config.sidebar.outer_margin);
        sidebar.set_margin_start(self.config.sidebar.inner_margin);
        sidebar.set_margin_end(self.config.sidebar.inner_margin);
        sidebar.set_valign(Align::Fill);

        for button in &self.config.sidebar.buttons {
            sidebar.append(&self.build_sidebar_button(button));
        }

        sidebar
    }

    fn build_sidebar_button(&self, button: &ButtonConfig) -> Button {
        let widget = Button::with_label(&button.label);
        widget.set_hexpand(true);
        widget.set_tooltip_text(Some(&Self::button_tooltip(button)));
        widget
    }

    fn build_right_panel(&self) -> GtkBox {
        let panel = GtkBox::new(Orientation::Vertical, self.config.grid.spacing);
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        panel.set_margin_top(self.config.header.outer_margin);
        panel.set_margin_bottom(self.config.header.outer_margin);
        panel.set_margin_start(8);
        panel.set_margin_end(self.config.header.outer_margin);

        panel.append(&self.build_header_banner());
        panel.append(&self.build_grid_section());
        panel
    }

    fn build_header_banner(&self) -> GtkBox {
        let banner = GtkBox::new(Orientation::Vertical, self.config.header.spacing);
        banner.set_hexpand(true);
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

        let subtitle = Label::new(Some(&self.config.header.subtitle));
        subtitle.set_halign(Align::Start);
        subtitle.set_xalign(0.0);
        subtitle.set_wrap(true);

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
        widget.set_tooltip_text(Some(&Self::button_tooltip(button)));
        widget
    }

    fn build_header_stat(&self, stat: &HeaderStatConfig) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 4);
        let label = Label::new(Some(&stat.label));
        let value = Label::new(Some(&stat.value));

        label.set_halign(Align::Start);
        label.set_xalign(0.0);
        value.set_halign(Align::Start);
        value.set_xalign(0.0);
        value.add_css_class("title-4");

        container.set_hexpand(true);
        container.append(&label);
        container.append(&value);
        container
    }

    fn build_grid_section(&self) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();

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
                &self.build_menu_tile_button(button, &self.config.grid),
                column,
                row,
                1,
                1,
            );
        }

        scroller.set_child(Some(&grid));
        scroller
    }

    fn build_menu_tile_button(&self, button: &ButtonConfig, grid: &GridSectionConfig) -> Button {
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

        content.append(&icon);
        content.append(&title);
        widget.set_child(Some(&content));
        widget
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
