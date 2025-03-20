use bluetooth::BluetoothService;
use components::{App, BluetoothButton, Window};
use gtk::{gio::prelude::ApplicationExtManual, glib::types::StaticType};
use once_cell::sync::Lazy;
use std::io::Write;

mod bluetooth;
mod components;

pub static BLUETOOTH: Lazy<BluetoothService> = Lazy::new(BluetoothService::new);

fn main() -> gtk::glib::ExitCode {
    setup_logger();
    setup_resources();
    register_custom_types();
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

fn register_custom_types() {
    BluetoothButton::static_type();
    Window::static_type();
    App::static_type();
}
