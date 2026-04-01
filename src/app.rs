use std::sync::Arc;

use gtk4::gdk::{self, Display};
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label,
    Orientation,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::config::{ButtonConfig, LauncherConfig, MenuAction};
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

        app.run();
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
        root.append(&self.build_main_panel());
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

    fn build_main_panel(&self) -> GtkBox {
        let content = GtkBox::new(Orientation::Vertical, self.config.main_panel.spacing);
        content.set_hexpand(true);
        content.set_vexpand(true);
        content.set_margin_top(self.config.main_panel.outer_margin);
        content.set_margin_bottom(self.config.main_panel.outer_margin);
        content.set_margin_start(8);
        content.set_margin_end(self.config.main_panel.outer_margin);

        let title = Label::new(Some(&self.config.main_panel.title));
        title.set_halign(Align::Start);
        title.add_css_class("title-2");

        let placeholder = Label::new(Some(&self.config.main_panel.description));
        placeholder.set_halign(Align::Start);
        placeholder.set_valign(Align::Start);
        placeholder.set_wrap(true);
        placeholder.set_xalign(0.0);
        placeholder.set_vexpand(true);

        content.append(&title);
        content.append(&placeholder);
        content
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
