mod database;

use anyhow::Ok;
use gtk::{Application, ApplicationWindow, prelude::*};

fn main() -> anyhow::Result<()> {
    let app = Application::builder()
        .application_id("me.bluegecko.coffeetime")
        .build();

    app.connect_activate(build_ui);
    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Coffeetime")
        .default_width(400)
        .default_height(300)
        .build();

    /*
    println!("{:?}", {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { database::get_today_data("sqlite://daemon/test.db").await })
    });
    */

    window.present();
}
