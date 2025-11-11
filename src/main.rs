mod database;

use adw::{ApplicationWindow, HeaderBar};
use anyhow::Ok;
use gtk::{Application, prelude::*};
use gtk::{Box, Label, Orientation};

fn main() -> anyhow::Result<()> {
    let app = Application::builder()
        .application_id("me.bluegecko.coffeetime")
        .build();

    app.connect_activate(build_ui);
    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    let header = HeaderBar::builder()
        .title_widget(&Label::new(Some("Coffeetime")))
        .build();

    let top_box = Box::builder()
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    top_box.append(&header);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(400)
        .content(&top_box)
        .build();

    window.present();
}
