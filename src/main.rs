use components::App;
use gtk::gio::prelude::ApplicationExtManual;

mod components;

fn main() -> gtk::glib::ExitCode {
    env_logger::builder().parse_default_env().filter_level(log::LevelFilter::Debug).init();

    gtk::gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    let app = App::new();
    app.run()
}
