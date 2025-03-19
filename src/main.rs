use components::App;
use gtk::gio::prelude::ApplicationExtManual;
use std::io::Write;

mod components;
mod bluetooth;

fn main() -> gtk::glib::ExitCode {
    setup_logger();
    setup_resources();

    let app = App::new();
    app.run()
}

fn setup_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let timestamp = buf.timestamp();
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{timestamp} : {level_style}{}{level_style:#} : {}] {}",
                record.level(),
                std::thread::current().name().unwrap_or("unnamed-thread"),
                record.args(),
            )
        })
        .filter_level(log::LevelFilter::Debug)
        .init();
}

fn setup_resources() {
    gtk::gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");
}
