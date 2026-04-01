use gtk4::gdk::{self, Display};
use gtk4::glib::Propagation;
use gtk4::glib::object::Cast as _;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "com.qrzbing.hyw-menu-gtk4";
const WINDOW_TITLE: &str = "hyw-menu";
const MIN_WIDTH: i32 = 320;

fn primary_monitor(display: &Display) -> Option<gdk::Monitor> {
    let monitors = display.monitors();
    let obj = monitors.item(0)?;
    obj.downcast::<gdk::Monitor>().ok()
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
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let display = Display::default().expect("No display available");
        let monitor = primary_monitor(&display).expect("No monitor available");

        let window = build_window(app, &monitor);
        configure_layer_shell(&window, &monitor);
        install_keybindings(&window);

        window.present();
    });

    app.run();
}
