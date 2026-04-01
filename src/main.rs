use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.qrzbing.hyw-menu-gtk4")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("hyw-menu")
            .default_width(480)
            .default_height(720)
            .build();

        window.present();
    });

    app.run();
}
