use std::env;
use std::sync::Arc;

use gtk4::gdk::{self, Display};
use gtk4::glib::Propagation;
use gtk4::glib::object::Cast as _;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use tokio::runtime::{Builder, Runtime};
use wayle_hyprland::HyprlandService;

const APP_ID: &str = "com.qrzbing.hyw-menu-gtk4";
const WINDOW_TITLE: &str = "hyw-menu";
const MIN_WIDTH: i32 = 320;

struct HyprlandContext {
    _runtime: Arc<Runtime>,
    service: Arc<HyprlandService>,
}

fn all_monitors(display: &Display) -> Vec<gdk::Monitor> {
    let monitors = display.monitors();

    (0..monitors.n_items())
        .filter_map(|index| monitors.item(index))
        .filter_map(|obj| obj.downcast::<gdk::Monitor>().ok())
        .collect()
}

fn first_monitor(display: &Display) -> Option<gdk::Monitor> {
    all_monitors(display).into_iter().next()
}

fn init_hyprland_context() -> Option<HyprlandContext> {
    if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_none() {
        return None;
    }

    let runtime = Builder::new_multi_thread().enable_all().build().ok()?;
    let service = runtime.block_on(HyprlandService::new()).ok()?;

    Some(HyprlandContext {
        _runtime: Arc::new(runtime),
        service,
    })
}

fn hyprland_focused_monitor_name(service: &HyprlandService) -> Option<String> {
    service
        .monitors
        .get()
        .into_iter()
        .find(|monitor| monitor.focused.get())
        .map(|monitor| monitor.name.get())
}

fn monitor_by_connector(display: &Display, connector: &str) -> Option<gdk::Monitor> {
    all_monitors(display)
        .into_iter()
        .find(|monitor| monitor.connector().as_deref() == Some(connector))
}

fn preferred_monitor(
    display: &Display,
    hyprland: Option<&HyprlandContext>,
) -> Option<gdk::Monitor> {
    if let Some(name) = hyprland.and_then(|ctx| hyprland_focused_monitor_name(&ctx.service)) {
        if let Some(monitor) = monitor_by_connector(display, &name) {
            return Some(monitor);
        }
    }

    first_monitor(display)
}

fn quarter_monitor_width(monitor: &gdk::Monitor) -> i32 {
    (monitor.geometry().width() / 4).max(MIN_WIDTH)
}

fn build_window(app: &Application, monitor: &gdk::Monitor) -> ApplicationWindow {
    let geometry = monitor.geometry();

    ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .default_width(quarter_monitor_width(monitor))
        .default_height(geometry.height())
        .build()
}

fn configure_layer_shell(window: &ApplicationWindow, monitor: &gdk::Monitor) {
    window.set_decorated(false);
    window.set_resizable(false);

    window.init_layer_shell();
    window.set_namespace(Some(WINDOW_TITLE));
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_monitor(Some(monitor));

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Right, false);

    window.set_margin(Edge::Left, 0);
    window.set_margin(Edge::Top, 0);
    window.set_margin(Edge::Bottom, 0);
    window.set_margin(Edge::Right, 0);

    // Reserve no screen space so Hyprland keeps other windows untouched.
    window.set_exclusive_zone(0);
}

fn install_keybindings(window: &ApplicationWindow) {
    let key_controller = EventControllerKey::new();
    let window_for_keys = window.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            window_for_keys.close();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    });

    window.add_controller(key_controller);
}

fn main() {
    let hyprland = init_hyprland_context();
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let display = Display::default().expect("No display available");
        let monitor = preferred_monitor(&display, hyprland.as_ref()).expect("No monitor available");

        let window = build_window(app, &monitor);
        configure_layer_shell(&window, &monitor);
        install_keybindings(&window);

        window.present();
    });

    app.run();
}
