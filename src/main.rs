mod data_processing;
mod database;
mod draw_graph;
mod utils;

use adw::{ApplicationWindow, HeaderBar};
use anyhow::Ok;
use gtk::{Application, prelude::*};
use gtk::{Box, Label, Orientation};

use crate::utils::get_humanized_uptime;

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
        .margin_top(16)
        .margin_start(30)
        .margin_end(0)
        .build();
    today_label.set_markup("<span font='12'>Today</span>");

    let today_screen_time_label = Label::builder()
        .halign(gtk::Align::Start)
        .margin_top(0)
        .margin_start(30)
        .margin_end(32)
        .build();

    today_screen_time_label.set_markup(
        &format!(
            "<span font='32'>{}</span>",
            data_processing::today_total_screen_time().unwrap_or("0h 0m".to_string())
        )
        .to_string(),
    );

    let uptime_label = Label::builder()
        .label(format!("Uptime: {}", get_humanized_uptime()))
        .halign(gtk::Align::Start)
        .margin_top(8)
        .margin_start(30)
        .margin_end(0)
        .build();

    let graph = draw_graph::draw_graph();

    top_box.append(&today_label);
    top_box.append(&today_screen_time_label);
    top_box.append(&uptime_label);
    top_box.append(&graph);

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
