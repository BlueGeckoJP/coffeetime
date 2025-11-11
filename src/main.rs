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
        .valign(gtk::Align::Start)
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    let today_label = Label::builder()
        .halign(gtk::Align::Start)
        .margin_top(8)
        .margin_start(16)
        .margin_end(0)
        .build();
    today_label.set_markup("<span font='12'>Today</span>");

    let today_screen_time_label = Label::builder()
        .halign(gtk::Align::Start)
        .margin_top(0)
        .margin_start(16)
        .margin_end(12)
        .build();

    today_screen_time_label.set_markup("<span font='32'>2h 30m</span>");

    top_box.append(&today_label);
    top_box.append(&today_screen_time_label);

    let base_box = Box::builder()
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    base_box.append(&header);
    base_box.append(&top_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(400)
        .content(&base_box)
        .build();

    window.present();
}
