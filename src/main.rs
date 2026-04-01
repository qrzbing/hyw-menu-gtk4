use gtk4::gdk::{self, Display};
use gtk4::glib::object::Cast as _;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

fn primary_monitor(display: &Display) -> Option<gdk::Monitor> {
    let monitors = display.monitors();
    let obj = monitors.item(0)?;
    obj.downcast::<gdk::Monitor>().ok()
}

fn main() {
    let app = Application::builder()
        .application_id("com.qrzbing.hyw-menu-gtk4")
        .build();

    app.connect_activate(|app| {
        let display = Display::default().expect("No display available");
        let monitor = primary_monitor(&display).expect("No monitor available");
        let geometry = monitor.geometry();
        let quarter_width = (geometry.width() / 4).max(320);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("hyw-menu")
            .default_width(quarter_width)
            .default_height(geometry.height())
            .build();

        window.set_decorated(false);
        window.set_resizable(false);

        window.init_layer_shell();
        window.set_namespace(Some("hyw-menu"));
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::OnDemand);
        window.set_monitor(Some(&monitor));

        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Right, false);

        window.set_margin(Edge::Left, 0);
        window.set_margin(Edge::Top, 0);
        window.set_margin(Edge::Bottom, 0);
        window.set_margin(Edge::Right, 0);

        // Do not reserve screen space: this window should overlay other apps
        // instead of affecting the compositor's tiling layout.
        window.set_exclusive_zone(0);

        window.present();
    });

    app.run();
}
