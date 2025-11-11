use gtk::{Application, ApplicationWindow, glib, prelude::*};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("me.bluegecko.coffeetime")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Coffee Time")
        .default_width(400)
        .default_height(300)
        .build();

    window.present();
}
