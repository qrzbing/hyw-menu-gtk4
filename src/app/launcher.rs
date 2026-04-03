use std::path::Path;
use std::sync::Arc;

use gst::prelude::*;
use gstreamer as gst;
use gtk4::gdk::Display;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Orientation,
    Picture, Stack, gdk,
};
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

const PAIMON_VIDEO_WIDTH: i32 = 1280;
const PAIMON_VIDEO_HEIGHT: i32 = 1800;

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
            gst::init().expect("failed to initialize gstreamer");
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
        window.add_css_class("launcher-window");

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
            .map_or_else(|| "?".to_owned(), |ch| ch.to_uppercase().to_string())
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
        let character_width = Self::character_video_width(monitor, config);
        let minimum_required = Self::minimum_required_width(config) + character_width;
        let ratio_width =
            ((monitor.geometry().width() as f32) * config.window().width_ratio()).round() as i32;
        let preferred = ratio_width
            .max(config.window().min_width())
            .max(Self::minimum_required_width(config))
            + character_width;

        match config.window().max_width() {
            Some(max_width) => preferred.min(max_width.max(minimum_required)),
            None => preferred,
        }
    }

    fn preferred_height(monitor: &gdk::Monitor, config: &LauncherConfig) -> i32 {
        ((monitor.geometry().height() as f32) * config.window().height_ratio()).round() as i32
    }

    fn minimum_required_width(config: &LauncherConfig) -> i32 {
        let sidebar_width = ((config.sidebar().width() as f32) * config.sidebar().scale())
            .round()
            .max(56.0) as i32;
        let panel_margins = 8;
        let columns = config.grid().columns().max(1);
        let grid_width =
            (config.grid().tile_size() * columns) + (config.grid().tile_spacing() * (columns - 1));

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

    fn character_video_width(monitor: &gdk::Monitor, config: &LauncherConfig) -> i32 {
        if Self::character_video_path(config).is_none() {
            return 0;
        }

        let video_height = ((Self::preferred_height(monitor, config) as f32)
            * config.character_video().height_ratio())
        .round() as i32;
        let video_width = (video_height * PAIMON_VIDEO_WIDTH) / PAIMON_VIDEO_HEIGHT;

        (video_width + config.character_video().offset_x().max(0)).max(0)
    }

    fn character_video_path(config: &LauncherConfig) -> Option<&Path> {
        config
            .character_video()
            .enabled()
            .then(|| config.character_video().path())
            .flatten()
            .filter(|path| path.is_file())
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

        let root = GtkBox::new(Orientation::Horizontal, 0);
        let menu_surface = GtkBox::new(Orientation::Horizontal, 0);

        root.add_css_class("launcher-root");
        root.set_hexpand(true);
        root.set_vexpand(true);

        menu_surface.add_css_class("launcher-menu-surface");
        menu_surface.append(&self.build_sidebar_nav(&navigator));
        menu_surface.append(&self.build_right_panel(&content_stack, &navigator, &all_apps));
        root.append(&menu_surface);

        if let Some(character_slot) = self.build_character_video_slot() {
            root.append(&character_slot);
        }

        navigator.activate_section(&self.initial_section_id());
        root
    }

    fn build_character_video_slot(&self) -> Option<GtkBox> {
        let video_path = Self::character_video_path(self.config())?;

        let video_height = ((self.window.default_height() as f32)
            * self.config().character_video().height_ratio())
        .round() as i32;
        let video_width = (video_height * PAIMON_VIDEO_WIDTH) / PAIMON_VIDEO_HEIGHT;
        let slot = GtkBox::new(Orientation::Vertical, 0);
        let (picture, playbin) =
            self.build_character_picture(video_path, video_width, video_height)?;

        self.install_character_player(playbin);
        slot.add_css_class("launcher-character-slot");
        slot.set_width_request(
            (video_width + self.config().character_video().offset_x().max(0)).max(0),
        );
        slot.set_hexpand(false);
        slot.set_vexpand(true);
        slot.set_halign(Align::End);
        slot.set_valign(Align::Fill);
        slot.set_margin_start(self.config().character_video().offset_x());
        picture.set_margin_bottom(self.config().character_video().offset_y());
        slot.append(&picture);

        Some(slot)
    }

    fn build_character_picture(
        &self,
        video_path: &Path,
        video_width: i32,
        video_height: i32,
    ) -> Option<(Picture, gst::Element)> {
        let sink = gst::ElementFactory::make("gtk4paintablesink")
            .property("window-width", video_width.cast_unsigned())
            .property("window-height", video_height.cast_unsigned())
            .build()
            .map_err(|error| {
                eprintln!("failed to create gtk4paintablesink: {error}");
                error
            })
            .ok()?;
        let playbin = gst::ElementFactory::make("playbin3")
            .property(
                "uri",
                gtk4::gio::File::for_path(video_path).uri().to_string(),
            )
            .property("video-sink", &sink)
            .property("mute", true)
            .build()
            .map_err(|error| {
                eprintln!("failed to create playbin3 for character video: {error}");
                error
            })
            .ok()?;
        let paintable = sink.property::<Option<gdk::Paintable>>("paintable")?;
        let picture = Picture::for_paintable(&paintable);

        picture.add_css_class("launcher-character-video");
        picture.set_halign(Align::End);
        picture.set_valign(Align::End);
        picture.set_focusable(false);
        picture.set_can_target(false);
        picture.set_hexpand(false);
        picture.set_vexpand(false);
        picture.set_width_request(video_width);
        picture.set_height_request(video_height);
        picture.set_can_shrink(true);

        if let Err(error) = playbin.set_state(gst::State::Playing) {
            eprintln!("failed to start character video pipeline: {error}");
            let _ = playbin.set_state(gst::State::Null);
            return None;
        }

        Some((picture, playbin))
    }

    fn install_character_player(&self, playbin: gst::Element) {
        self.window.connect_close_request(move |_| {
            let _ = playbin.set_state(gst::State::Null);
            Propagation::Proceed
        });
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
