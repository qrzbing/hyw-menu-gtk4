use std::sync::Arc;

use gtk4::gdk::Display;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Stack, gdk};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use super::catalog::DesktopAppCatalog;
use super::navigation::LauncherNavigator;
use super::pages::AllAppsPageState;
use super::quick_access::QuickAccessState;
use crate::config::{ButtonConfig, LauncherConfig, MenuAction};
use crate::hyprland::{HyprlandContext, preferred_monitor};
use crate::style::install_global_css;

pub struct LauncherApp {
    config: Arc<LauncherConfig>,
    hyprland: Option<HyprlandContext>,
}

pub(crate) struct LauncherWindow {
    window: ApplicationWindow,
    config: Arc<LauncherConfig>,
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
            .application_id(config.application_id())
            .build();

        app.connect_activate(move |application| {
            let display = Display::default().expect("no display available");
            install_global_css(&display, &config);
            let monitor =
                preferred_monitor(&display, hyprland.as_ref(), config.window().min_width())
                    .expect("no monitor available");

            let launcher_window = LauncherWindow::new(application, &monitor, config.clone());
            launcher_window.present();
        });

        app.run_with_args::<&str>(&[]);
    }
}

impl LauncherWindow {
    pub(crate) fn new(
        app: &Application,
        monitor: &gdk::Monitor,
        config: Arc<LauncherConfig>,
    ) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(config.window().title())
            .default_width(Self::preferred_width(monitor, &config))
            .default_height(Self::preferred_height(monitor, &config))
            .build();

        let launcher_window = Self { window, config };
        launcher_window.configure_layer_shell(monitor);
        launcher_window.install_keybindings();
        launcher_window.mount_content();
        launcher_window
    }

    pub(crate) fn config(&self) -> &LauncherConfig {
        self.config.as_ref()
    }

    pub(crate) fn present(&self) {
        self.window.present();
    }

    pub(crate) fn bind_button_action(
        &self,
        widget: &Button,
        button: &ButtonConfig,
        navigator: Option<LauncherNavigator>,
    ) {
        let action = button.action().clone();
        let window = self.window.clone();

        widget.connect_clicked(move |_| match &action {
            MenuAction::CloseMenu => window.close(),
            MenuAction::OpenSection(section) => {
                if let Some(navigator) = &navigator {
                    navigator.activate_section(section);
                }
            }
            MenuAction::None => {}
        });
    }

    pub(crate) fn button_tooltip(button: &ButtonConfig) -> String {
        let icon = button.icon_name().unwrap_or("none");

        match button.action() {
            MenuAction::None => format!("{} ({icon})", button.id()),
            MenuAction::CloseMenu => format!("{} -> close ({icon})", button.id()),
            MenuAction::OpenSection(section) => {
                format!("{} -> section:{section} ({icon})", button.id())
            }
        }
    }

    pub(crate) fn sidebar_button_size(&self) -> i32 {
        ((self.sidebar_width() as f32) * self.config().sidebar().button_scale())
            .round()
            .clamp(40.0, self.sidebar_width() as f32) as i32
    }

    pub(crate) fn sidebar_icon_fallback_text(button: &ButtonConfig) -> String {
        button
            .label()
            .chars()
            .next()
            .map(|ch| ch.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_owned())
    }

    pub(crate) fn sidebar_middle_spacer_height(&self) -> i32 {
        (self.visible_top_panels_height() + self.config().grid().spacing()
            - self.sidebar_button_size()
            + self.config().sidebar().middle_offset())
        .max(0)
    }

    pub(crate) fn sidebar_width(&self) -> i32 {
        ((self.config().sidebar().width() as f32) * self.config().sidebar().scale())
            .round()
            .max(56.0) as i32
    }

    fn preferred_width(monitor: &gdk::Monitor, config: &LauncherConfig) -> i32 {
        let ratio_width =
            ((monitor.geometry().width() as f32) * config.window().width_ratio()).round() as i32;
        ratio_width
            .max(config.window().min_width())
            .max(Self::minimum_required_width(config))
    }

    fn preferred_height(monitor: &gdk::Monitor, config: &LauncherConfig) -> i32 {
        ((monitor.geometry().height() as f32) * config.window().height_ratio()).round() as i32
    }

    fn minimum_required_width(config: &LauncherConfig) -> i32 {
        let sidebar_width = ((config.sidebar().width() as f32) * config.sidebar().scale())
            .round()
            .max(56.0) as i32;
        let panel_margins = 8 + config.top_panels().outer_margin();
        let columns = config.grid().columns().max(1);
        let grid_width =
            (config.grid().tile_size() * columns) + (config.grid().spacing() * (columns - 1));

        sidebar_width + panel_margins + grid_width + 24
    }

    fn visible_top_panels_height(&self) -> i32 {
        if self.config().top_panels().left_panels().is_empty()
            && self.config().top_panels().right_panels().is_empty()
        {
            0
        } else {
            self.config().top_panels().height()
        }
    }

    fn mount_content(&self) {
        let root = self.build_root_container();
        self.window.set_child(Some(&root));
    }

    fn build_root_container(&self) -> GtkBox {
        let catalog = DesktopAppCatalog::collect();
        let quick_access = QuickAccessState::new(
            self.config().quick_access_path().to_path_buf(),
            catalog.clone(),
            self.config.clone(),
            &self.window,
        );
        let all_apps = AllAppsPageState::new(
            catalog,
            quick_access.clone(),
            self.config.clone(),
            &self.window,
        );
        let content_stack = Stack::new();
        let navigator = LauncherNavigator::new(&content_stack);
        self.populate_content_stack(&content_stack, &quick_access, &all_apps);

        let root = GtkBox::new(gtk4::Orientation::Horizontal, 0);
        root.add_css_class("launcher-root");
        root.append(&self.build_sidebar_nav(&navigator));
        root.append(&self.build_right_panel(&content_stack, &navigator, &all_apps));
        navigator.activate_section(&self.initial_section_id());
        root
    }

    fn initial_section_id(&self) -> String {
        self.config()
            .sidebar()
            .buttons()
            .iter()
            .find_map(|button| match button.action() {
                MenuAction::OpenSection(section) => Some(section.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "dashboard".to_owned())
    }

    fn configure_layer_shell(&self, monitor: &gdk::Monitor) {
        self.window.set_decorated(false);
        self.window.set_resizable(false);

        self.window.init_layer_shell();
        self.window
            .set_namespace(Some(self.config().window().namespace()));
        self.window.set_layer(Layer::Overlay);
        self.window.set_keyboard_mode(KeyboardMode::Exclusive);
        self.window.set_monitor(Some(monitor));

        self.window.set_anchor(Edge::Left, true);
        self.window.set_anchor(Edge::Top, true);
        self.window.set_anchor(Edge::Bottom, false);
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
}
