use std::path::Path;
use std::sync::Arc;

use gst::prelude::*;
use gstreamer as gst;
use gtk4::gdk::Display;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Orientation,
    Overflow, Picture, Stack, gdk,
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

struct CharacterWindow {
    window: ApplicationWindow,
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
            if let Some(character_window) =
                CharacterWindow::new(application, &monitor, config.clone())
            {
                launcher_window.bind_companion_close(&character_window.window);
                character_window.present();
            }
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

    fn bind_companion_close(&self, companion: &ApplicationWindow) {
        let companion = companion.clone();
        self.window.connect_close_request(move |_| {
            companion.close();
            Propagation::Proceed
        });
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
        let minimum_required = Self::minimum_required_width(config);
        let ratio_width =
            ((monitor.geometry().width() as f32) * config.window().width_ratio()).round() as i32;
        let preferred = ratio_width
            .max(config.window().min_width())
            .max(Self::minimum_required_width(config));

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
        if !Self::should_show_character_window(config) {
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
        root.set_widget_name("launcher-root");
        root.set_hexpand(true);
        root.set_vexpand(true);

        menu_surface.add_css_class("launcher-menu-surface");
        menu_surface.set_hexpand(true);
        menu_surface.set_vexpand(true);
        menu_surface.set_halign(Align::Fill);
        menu_surface.set_valign(Align::Fill);
        menu_surface.append(&self.build_sidebar_nav(&navigator));
        menu_surface.append(&self.build_right_panel(&content_stack, &navigator, &all_apps));
        root.append(&menu_surface);

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
        self.window.set_widget_name("launcher-window");

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

impl CharacterWindow {
    fn new(app: &Application, monitor: &gdk::Monitor, config: Arc<LauncherConfig>) -> Option<Self> {
        if !LauncherWindow::should_show_character_window(&config) {
            return None;
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title(config.window().title())
            .default_width(LauncherWindow::character_video_width(monitor, &config))
            .default_height(LauncherWindow::preferred_height(monitor, &config))
            .build();
        window.add_css_class("launcher-window");
        window.add_css_class("launcher-character-window");

        let character_window = Self { window };
        character_window.configure_layer_shell(
            monitor,
            &config,
            LauncherWindow::preferred_width(monitor, &config),
        );
        character_window.mount_content(&config);
        Some(character_window)
    }

    fn present(&self) {
        self.window.present();
    }

    fn configure_layer_shell(
        &self,
        monitor: &gdk::Monitor,
        config: &LauncherConfig,
        margin_left: i32,
    ) {
        self.window.set_decorated(false);
        self.window.set_resizable(false);
        self.window.set_widget_name("launcher-character-window");
        self.window.set_focusable(false);
        self.window.set_can_target(false);

        self.window.init_layer_shell();
        self.window
            .set_namespace(Some(&format!("{}-character", config.window().namespace())));
        self.window.set_layer(Layer::Overlay);
        self.window.set_keyboard_mode(KeyboardMode::None);
        self.window.set_monitor(Some(monitor));

        self.window.set_anchor(Edge::Left, true);
        self.window.set_anchor(Edge::Top, true);
        self.window.set_anchor(Edge::Bottom, false);
        self.window.set_anchor(Edge::Right, false);

        self.window.set_margin(Edge::Left, margin_left);
        self.window.set_margin(Edge::Top, 0);
        self.window.set_margin(Edge::Bottom, 0);
        self.window.set_margin(Edge::Right, 0);

        self.window.set_exclusive_zone(0);
    }

    fn mount_content(&self, config: &LauncherConfig) {
        let Some(slot) = self.build_character_slot(config) else {
            return;
        };
        self.window.set_child(Some(&slot));
    }

    fn build_character_slot(&self, config: &LauncherConfig) -> Option<GtkBox> {
        let video_height = ((self.window.default_height() as f32)
            * config.character_video().height_ratio())
        .round() as i32;
        let video_width = (video_height * PAIMON_VIDEO_WIDTH) / PAIMON_VIDEO_HEIGHT;
        let slot = GtkBox::new(Orientation::Vertical, 0);
        slot.add_css_class("launcher-character-slot");
        slot.set_width_request((video_width + config.character_video().offset_x().max(0)).max(0));
        slot.set_hexpand(false);
        slot.set_vexpand(true);
        slot.set_halign(Align::Start);
        slot.set_valign(Align::Fill);
        slot.set_margin_start(config.character_video().offset_x());

        if config.character_video().outline_only() {
            let outline = self.build_character_outline(video_width, video_height);
            outline.set_margin_bottom(config.character_video().offset_y());
            slot.append(&outline);
            return Some(slot);
        }

        let video_path = LauncherWindow::character_video_path(config)?;
        let (picture, playbin) =
            self.build_character_picture(video_path, video_width, video_height)?;
        self.install_character_player(playbin);
        picture.set_margin_bottom(config.character_video().offset_y());
        slot.append(&picture);

        Some(slot)
    }

    fn build_character_outline(&self, video_width: i32, video_height: i32) -> GtkBox {
        let outline = GtkBox::new(Orientation::Vertical, 0);
        outline.add_css_class("launcher-character-outline");
        outline.set_halign(Align::Start);
        outline.set_valign(Align::End);
        outline.set_focusable(false);
        outline.set_can_target(false);
        outline.set_hexpand(false);
        outline.set_vexpand(false);
        outline.set_width_request(video_width);
        outline.set_height_request(video_height);
        outline.set_overflow(Overflow::Hidden);
        outline
    }

    fn build_character_picture(
        &self,
        video_path: &Path,
        video_width: i32,
        video_height: i32,
    ) -> Option<(Picture, gst::Element)> {
        if let Some(result) =
            self.build_alpha_character_picture(video_path, video_width, video_height)
        {
            return Some(result);
        }

        self.build_standard_character_picture(video_path, video_width, video_height)
    }

    fn build_alpha_character_picture(
        &self,
        video_path: &Path,
        video_width: i32,
        video_height: i32,
    ) -> Option<(Picture, gst::Element)> {
        if video_path.extension().and_then(|ext| ext.to_str()) != Some("webm") {
            return None;
        }

        let pipeline_description = format!(
            "filesrc location=\"{}\" ! matroskademux name=demux demux.video_0 ! queue ! decodebin ! videoconvert ! video/x-raw,format=RGBA ! gtk4paintablesink name=character_sink window-width={} window-height={}",
            video_path.display(),
            video_width,
            video_height
        );
        let pipeline = gst::parse::launch(&pipeline_description)
            .map_err(|error| {
                eprintln!("failed to create alpha pipeline for character video: {error}");
                error
            })
            .ok()?;
        let bin = pipeline.dynamic_cast_ref::<gst::Bin>()?;
        let sink = bin.by_name("character_sink")?;
        let paintable = sink.property::<Option<gdk::Paintable>>("paintable")?;
        let picture = self.configure_character_picture(&paintable, video_width, video_height);

        if let Err(error) = pipeline.set_state(gst::State::Playing) {
            eprintln!("failed to start alpha character video pipeline: {error}");
            let _ = pipeline.set_state(gst::State::Null);
            return None;
        }

        Some((picture, pipeline))
    }

    fn build_standard_character_picture(
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
        let picture = self.configure_character_picture(&paintable, video_width, video_height);

        if let Err(error) = playbin.set_state(gst::State::Playing) {
            eprintln!("failed to start character video pipeline: {error}");
            let _ = playbin.set_state(gst::State::Null);
            return None;
        }

        Some((picture, playbin))
    }

    fn configure_character_picture(
        &self,
        paintable: &gdk::Paintable,
        video_width: i32,
        video_height: i32,
    ) -> Picture {
        let picture = Picture::for_paintable(paintable);
        picture.add_css_class("launcher-character-video");
        picture.set_halign(Align::Start);
        picture.set_valign(Align::End);
        picture.set_focusable(false);
        picture.set_can_target(false);
        picture.set_hexpand(false);
        picture.set_vexpand(false);
        picture.set_width_request(video_width);
        picture.set_height_request(video_height);
        picture.set_can_shrink(true);
        picture
    }

    fn install_character_player(&self, playbin: gst::Element) {
        self.window.connect_close_request(move |_| {
            let _ = playbin.set_state(gst::State::Null);
            Propagation::Proceed
        });
    }
}

impl LauncherWindow {
    fn should_show_character_window(config: &LauncherConfig) -> bool {
        if !config.character_video().enabled() {
            return false;
        }

        config.character_video().outline_only() || Self::character_video_path(config).is_some()
    }
}
